use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-icon-button.js")]
extern "C" {
    #[derive(Debug)]
    type IconButton;

    #[wasm_bindgen(getter, static_method_of = IconButton)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(IconButton);

/// Props for [`MatIconButton`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button#propertiesattributes)
#[derive(Debug, Properties, Clone)]
pub struct IconButtonProps {
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub label: Cow<'static, str>,
    #[prop_or_default]
    pub icon: Cow<'static, str>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

component!(
    MatIconButton,
    IconButtonProps,
    |props: &IconButtonProps| {
        html! {
            <mwc-icon-button
                class=props.classes.clone()
                label=props.label.clone()
                icon=props.icon.clone()
                disabled=props.disabled
            >{ props.children.clone() }</mwc-icon-button>
        }
    },
    IconButton,
    "icon-button"
);
