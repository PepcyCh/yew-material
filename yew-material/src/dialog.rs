mod dialog_action;

pub use dialog_action::*;

use crate::{bool_to_option, event_details_into, WeakComponentLink};
use gloo::events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-dialog.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Dialog;

    #[wasm_bindgen(getter, static_method_of = Dialog)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method)]
    fn focus(this: &Dialog);

    #[wasm_bindgen(method)]
    fn blur(this: &Dialog);

    #[wasm_bindgen(method)]
    fn show(this: &Dialog);

    #[wasm_bindgen(method)]
    fn close(this: &Dialog);
}

loader_hack!(Dialog);

/// The `mwc-dialog` component.
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/dialog)
///
/// ## Actions
///
/// In order to pass actions, [`MatDialogAction`] component should be
/// used.
pub struct MatDialog {
    props: DialogProps,
    node_ref: NodeRef,
    opening_listener: Option<EventListener>,
    opened_listener: Option<EventListener>,
    closing_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
}

/// Props for [`MatDialog`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/dialog#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/dialog#events)
#[derive(Properties, Clone)]
pub struct DialogProps {
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub hide_action: bool,
    #[prop_or_default]
    pub stacked: bool,
    #[prop_or_default]
    pub heading: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub scrim_click_action: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub escape_key_action: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub default_action: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub action_attribute: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub initial_focus_attribute: Option<Cow<'static, str>>,
    /// Binds to `opening` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopening: Callback<()>,
    /// Binds to `opened` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `closing` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosing: Callback<String>,
    /// Binds to `closed` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<String>,
    /// [`WeakComponentLink`] for `MatDialog` which provides the following
    /// methods:
    /// - ```focus(&self)```
    /// - ```blur(&self)```
    /// - ```show(&self)```
    /// - ```close(&self)```
    ///
    /// See [`WeakComponentLink`] documentation for more information
    #[prop_or_default]
    pub dialog_link: WeakComponentLink<MatDialog>,
    pub children: Children,
}

impl Component for MatDialog {
    type Message = ();
    type Properties = DialogProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.dialog_link.borrow_mut().replace(link);
        Dialog::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            opening_listener: None,
            opened_listener: None,
            closing_listener: None,
            closed_listener: None,
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
        <mwc-dialog
            class=self.props.classes.clone()
            open=self.props.open
            hideActions=bool_to_option(self.props.hide_action)
            stacked=bool_to_option(self.props.stacked)
            heading=self.props.heading.clone()
            scrimClickAction=self.props.scrim_click_action.clone()
            escapeKeyAction=self.props.escape_key_action.clone()
            defaultAction=self.props.default_action.clone()
            actionAttribute=self.props.action_attribute.clone()
            initialFocusAttribute=self.props.initial_focus_attribute.clone()
            ref=self.node_ref.clone()>
            { self.props.children.clone() }
        </mwc-dialog>
                }
    }

    fn rendered(&mut self, _first_render: bool) {
        let element = self.node_ref.cast::<Element>().unwrap();
        if self.opening_listener.is_none() {
            let onopening = self.props.onopening.clone();
            self.opening_listener = Some(EventListener::new(&element, "opening", move |_| {
                onopening.emit(())
            }));
        }

        if self.opened_listener.is_none() {
            let onopened = self.props.onopened.clone();
            self.opened_listener = Some(EventListener::new(&element, "opened", move |_| {
                onopened.emit(())
            }));
        }

        if self.closing_listener.is_none() {
            let onclosing = self.props.onclosing.clone();
            self.closing_listener = Some(EventListener::new(&element, "closing", move |event| {
                onclosing.emit(action_from_event(event))
            }));
        }

        if self.closed_listener.is_none() {
            let onclosed = self.props.onclosed.clone();
            self.closed_listener = Some(EventListener::new(&element, "closed", move |event| {
                onclosed.emit(action_from_event(event))
            }));
        }
    }
}

impl WeakComponentLink<MatDialog> {
    pub fn focus(&self) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .focus()
    }

    pub fn blur(&self) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .blur()
    }

    pub fn show(&self) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .show()
    }

    pub fn close(&self) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .close()
    }
}

#[wasm_bindgen]
extern "C" {
    type DialogActionType;

    #[wasm_bindgen(method, getter)]
    fn action(this: &DialogActionType) -> String;
}

fn action_from_event(event: &Event) -> String {
    event_details_into::<DialogActionType>(event).action()
}
