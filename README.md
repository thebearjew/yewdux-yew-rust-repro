```
error[E0277]: expected a `Fn<(MouseEvent,)>` closure, found `yew::callback::Callback<MouseEvent>`
   --> src/modal.rs:67:34
    |
64  | /             html! {
65  | |                 <>
66  | |                     <div
67  | |                         onclick={&on_close_modal}
    | |                                  ^^^^^^^^^^^^^^^ expected an `Fn<(MouseEvent,)>` closure, found `yew::callback::Callback<MouseEvent>`
...   |
80  | |                 </>
81  | |             }
    | |_____________- required by a bound introduced by this call
    |
    = help: the trait `Fn<(MouseEvent,)>` is not implemented for `yew::callback::Callback<MouseEvent>`
    = note: required because of the requirements on the impl of `FnOnce<(MouseEvent,)>` for `&yew::callback::Callback<MouseEvent>`
    = note: required because of the requirements on the impl of `IntoEventCallback<MouseEvent>` for `&yew::callback::Callback<MouseEvent>`
note: required by a bound in `yew::html::onclick::Wrapper::__macro_new`
   --> /Users/CASE/.cargo/git/checkouts/yew-7424ad4d701b481c/cf0f59a/packages/yew/src/html/listener/events.rs:132:1
    |
132 | / impl_short! {
133 | |     onauxclick(MouseEvent)
134 | |     onclick(MouseEvent)
135 | |
...   |
196 | |     ontransitionstart(TransitionEvent)
197 | | }
    | |_^ required by this bound in `yew::html::onclick::Wrapper::__macro_new`
    = note: this error originates in the macro `impl_action` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `yewdux::component::WithDispatch<ModalBase>: yew::Component` is not satisfied
  --> src/main.rs:27:10
   |
27 |         <Modal>
   |          ^^^^^ the trait `yew::Component` is not implemented for `yewdux::component::WithDispatch<ModalBase>`

error[E0599]: the function or associated item `new` exists for struct `VChild<yewdux::component::WithDispatch<ModalBase>>`, but its trait bounds were not satisfied
  --> src/main.rs:27:10
   |
27 |           <Modal>
   |            ^^^^^ function or associated item cannot be called on `VChild<yewdux::component::WithDispatch<ModalBase>>` due to unsatisfied trait bounds
   |
  ::: /Users/CASE/.cargo/git/checkouts/yewdux-cc25a1d21ecd63a8/c89a56c/yewdux/src/component/wrapper.rs:30:1
   |
30 | / pub struct WithDispatch<C>
31 | | where
32 | |     C: Component,
33 | |     C::Properties: WithDispatchProps + Clone,
34 | | {
35 | |     state: Option<Rc<<<C::Properties as WithDispatchProps>::Store as Store>::Model>>,
36 | | }
   | |_- doesn't satisfy `_: yew::Component`
   |
   = note: the following trait bounds were not satisfied:
           `yewdux::component::WithDispatch<ModalBase>: yew::Component`

error[E0277]: the trait bound `ModalBase: yew::html::component::Component` is not satisfied
  --> src/main.rs:27:10
   |
27 |         <Modal>
   |          ^^^^^ the trait `yew::html::component::Component` is not implemented for `ModalBase`
   |
note: required by a bound in `yewdux::component::WithDispatch`
  --> /Users/CASE/.cargo/git/checkouts/yewdux-cc25a1d21ecd63a8/c89a56c/yewdux/src/component/wrapper.rs:32:8
   |
32 |     C: Component,
   |        ^^^^^^^^^ required by this bound in `yewdux::component::WithDispatch`

warning: unused import: `wasm_bindgen::JsCast`
 --> src/modal.rs:3:5
  |
3 | use wasm_bindgen::JsCast;
  |     ^^^^^^^^^^^^^^^^^^^^

Some errors have detailed explanations: E0277, E0432, E0599.
For more information about an error, try `rustc --explain E0277`.
```
