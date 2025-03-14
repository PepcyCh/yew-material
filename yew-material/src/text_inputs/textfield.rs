use super::set_on_input_handler;
use crate::bool_to_option;
use crate::text_inputs::{
    validity_state::ValidityStateJS, TextFieldType, ValidityState, ValidityTransform,
};
use gloo::events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
use web_sys::ValidityState as NativeValidityState;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-textfield.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type TextField;

    #[wasm_bindgen(getter, static_method_of = TextField)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(
        this: &TextField,
        val: &Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>,
    );

    #[wasm_bindgen(method, setter)]
    fn set_type(this: &TextField, val: &JsValue);

    #[wasm_bindgen(method, getter)]
    fn value(this: &TextField) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_value(this: &TextField, val: &JsValue);
}

loader_hack!(TextField);

/// The `mwc-textfield` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/textfield)
pub struct MatTextField {
    props: TextFieldProps,
    node_ref: NodeRef,
    validity_transform_closure:
        Option<Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>>,
    input_listener: Option<EventListener>,
}

/// Props for [`MatTextField`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/textfield#propertiesattributes)
#[derive(Properties, Clone)]
pub struct TextFieldProps {
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub value: Cow<'static, str>,
    #[prop_or(TextFieldType::Text)]
    pub field_type: TextFieldType,
    #[prop_or_default]
    pub label: Cow<'static, str>,
    #[prop_or_default]
    pub placeholder: Cow<'static, str>,
    #[prop_or_default]
    pub prefix: Cow<'static, str>,
    #[prop_or_default]
    pub suffix: Cow<'static, str>,
    #[prop_or_default]
    pub icon: Cow<'static, str>,
    #[prop_or_default]
    pub icon_trailing: Cow<'static, str>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub char_counter: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub helper: Cow<'static, str>,
    #[prop_or_default]
    pub helper_persistent: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub max_length: Option<u64>,
    #[prop_or_default]
    pub validation_message: Cow<'static, str>,
    #[prop_or_default]
    pub pattern: Cow<'static, str>,
    /// Type: `number | string` so I'll leave it as a string
    #[prop_or_default]
    pub min: Cow<'static, str>,
    /// Type: `number | string`  so I'll leave it as a string
    #[prop_or_default]
    pub max: Cow<'static, str>,
    // What you doing...
    #[prop_or_default]
    pub size: Option<i64>,
    // ...step size
    #[prop_or_default]
    pub step: Option<i64>,
    #[prop_or_default]
    pub auto_validate: bool,
    #[prop_or_default]
    pub validity_transform: Option<ValidityTransform>,
    #[prop_or_default]
    pub validate_on_initial_render: bool,
    #[prop_or_default]
    pub oninput: Callback<InputData>,
    #[prop_or_default]
    pub name: Cow<'static, str>,
}

impl Component for MatTextField {
    type Message = ();
    type Properties = TextFieldProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TextField::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            validity_transform_closure: None,
            input_listener: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-textfield
                class=self.props.classes.clone()
                open=self.props.open
                label=self.props.label.clone()
                placeholder=self.props.placeholder.clone()
                prefix=self.props.prefix.clone()
                suffix=self.props.suffix.clone()
                icon=self.props.icon.clone()
                iconTrailing=self.props.icon_trailing.clone()
                disabled=self.props.disabled
                charCounter=bool_to_option(self.props.char_counter)
                outlined=bool_to_option(self.props.outlined)
                helper=self.props.helper.clone()
                helperPersistent=bool_to_option(self.props.helper_persistent)
                required=self.props.required
                maxlength=self.props.max_length.map(|v| Cow::from(v.to_string()))
                validationMessage=self.props.validation_message.clone()
                pattern=self.props.pattern.clone()
                min=self.props.min.clone()
                max=self.props.max.clone()
                size=self.props.size.map(|v| Cow::from(v.to_string()))
                step=self.props.step.map(|v| Cow::from(v.to_string()))
                autoValidate=bool_to_option(self.props.auto_validate)
                validateOnInitialRender=bool_to_option(self.props.validate_on_initial_render)
                name=self.props.name.clone()
                ref=self.node_ref.clone()
            ></mwc-textfield>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<TextField>().unwrap();
        element.set_type(&JsValue::from(
            self.props.field_type.to_cow_string().as_ref(),
        ));
        element.set_value(&JsValue::from(self.props.value.as_ref()));

        if self.input_listener.is_none() {
            self.input_listener = Some(set_on_input_handler(
                &self.node_ref,
                self.props.oninput.clone(),
                |(input_event, detail)| {
                    InputData {
                        value: detail
                            .unchecked_into::<MatTextFieldInputEvent>()
                            .target()
                            .value(),
                        event: input_event,
                    }
                },
            ));
        }
        if first_render {
            let this = self.node_ref.cast::<TextField>().unwrap();
            if let Some(transform) = self.props.validity_transform.clone() {
                self.validity_transform_closure = Some(Closure::wrap(Box::new(
                    move |s: String, v: NativeValidityState| -> ValidityStateJS {
                        transform.0(s, v).into()
                    },
                )
                    as Box<dyn Fn(String, NativeValidityState) -> ValidityStateJS>));
                this.set_validity_transform(&self.validity_transform_closure.as_ref().unwrap());
            }
        }
    }
}

impl MatTextField {
    pub fn validity_transform<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(
        func: F,
    ) -> ValidityTransform {
        ValidityTransform::new(func)
    }
}

#[wasm_bindgen]
extern "C" {
    type MatTextFieldInputEvent;

    #[wasm_bindgen(method, getter)]
    fn target(this: &MatTextFieldInputEvent) -> TextField;
}
