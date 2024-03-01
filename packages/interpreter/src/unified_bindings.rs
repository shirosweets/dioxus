#[cfg(feature = "webonly")]
use web_sys::Node;

#[cfg(feature = "webonly")]
use wasm_bindgen::prelude::wasm_bindgen;

use sledgehammer_bindgen::bindgen;

pub const SLEDGEHAMMER_JS: &str = GENERATED_JS;

/// Extensions to the interpreter that are specific to the web platform.
#[cfg(feature = "webonly")]
#[wasm_bindgen(module = "src/js/web.js")]
extern "C" {
    pub type WebInterpreter;

    #[wasm_bindgen(method, js_name = "saveTemplate")]
    pub fn save_template(this: &WebInterpreter, nodes: Vec<Node>, tmpl_id: u16);

    #[wasm_bindgen(method)]
    pub fn hydrate(this: &WebInterpreter, ids: Vec<u32>);

    #[wasm_bindgen(method, js_name = "getNode")]
    pub fn get_node(this: &WebInterpreter, id: u32) -> Node;
}

#[cfg(feature = "webonly")]
type PlatformInterpreter = WebInterpreter;

#[cfg(feature = "webonly")]
impl Interpreter {
    /// Convert the interpreter to a web interpreter, enabling methods like hydrate and save_template.
    pub fn as_web(&self) -> &WebInterpreter {
        use wasm_bindgen::prelude::JsCast;
        &self.js_channel().unchecked_ref()
    }
}

#[bindgen(module)]
mod js {
    /// The interpreter extends the core interpreter which contains the state for the interpreter along with some functions that all platforms use like `AppendChildren`.
    #[extends(PlatformInterpreter)]
    pub struct Interpreter;

    fn mount_to_root() {
        "{this.AppendChildren(this.root, this.stack.length-1);}"
    }
    fn push_root(root: u32) {
        "{this.stack.push(this.nodes[$root$]);}"
    }
    fn append_children(id: u32, many: u16) {
        "{this.AppendChildren($id$, $many$);}"
    }
    fn pop_root() {
        "{this.stack.pop();}"
    }
    fn replace_with(id: u32, n: u16) {
        "{const root = this.nodes[$id$]; this.els = this.stack.splice(this.stack.length-$n$); if (root.listening) { this.listeners.removeAllNonBubbling(root); } root.replaceWith(...this.els);}"
    }
    fn insert_after(id: u32, n: u16) {
        "{this.nodes[$id$].after(...this.stack.splice(this.stack.length-$n$));}"
    }
    fn insert_before(id: u32, n: u16) {
        "{this.nodes[$id$].before(...this.stack.splice(this.stack.length-$n$));}"
    }
    fn remove(id: u32) {
        "{let node = this.nodes[$id$]; if (node !== undefined) { if (node.listening) { this.listeners.removeAllNonBubbling(node); } node.remove(); }}"
    }
    fn create_raw_text(text: &str) {
        "{this.stack.push(document.createTextNode($text$));}"
    }
    fn create_text_node(text: &str, id: u32) {
        "{let node = document.createTextNode($text$); this.nodes[$id$] = node; this.stack.push(node);}"
    }
    fn create_placeholder(id: u32) {
        "{let node = document.createElement('pre'); node.hidden = true; this.stack.push(node); this.nodes[$id$] = node;}"
    }
    fn new_event_listener(event_name: &str<u8, evt>, id: u32, bubbles: u8) {
        r#"let node = this.nodes[id]; if(node.listening){node.listening += 1;}else{node.listening = 1;} node.setAttribute('data-dioxus-id', `\${id}`); this.listeners.create($event_name$, node, $bubbles$);"#
    }
    fn remove_event_listener(event_name: &str<u8, evt>, id: u32, bubbles: u8) {
        "{let node = this.nodes[$id$]; node.listening -= 1; node.removeAttribute('data-dioxus-id'); this.listeners.remove(node, $event_name$, $bubbles$);}"
    }
    fn set_text(id: u32, text: &str) {
        "{this.nodes[$id$].textContent = $text$;}"
    }
    fn set_attribute(id: u32, field: &str<u8, attr>, value: &str, ns: &str<u8, ns_cache>) {
        "{let node = this.nodes[$id$]; this.setAttributeInner(node, $field$, $value$, $ns$);}"
    }
    fn remove_attribute(id: u32, field: &str<u8, attr>, ns: &str<u8, ns_cache>) {
        r#"{
            let node = this.nodes[$id$];
            if (!ns) {
                switch (field) {
                    case "value":
                        node.value = "";
                        break;
                    case "checked":
                        node.checked = false;
                        break;
                    case "selected":
                        node.selected = false;
                        break;
                    case "dangerous_inner_html":
                        node.innerHTML = "";
                        break;
                    default:
                        node.removeAttribute(field);
                        break;
                }
            } else if (ns == "style") {
                node.style.removeProperty(name);
            } else {
                node.removeAttributeNS(ns, field);
            }
        }"#
    }
    fn assign_id(ptr: u32, len: u8, id: u32) {
        "{this.nodes[$id$] = this.LoadChild($ptr$, $len$);}"
    }
    fn hydrate_text(ptr: u32, len: u8, value: &str, id: u32) {
        r#"{
            let node = this.LoadChild($ptr$, $len$);
            if (node.nodeType == node.TEXT_NODE) {
                node.textContent = value;
            } else {
                let text = document.createTextNode(value);
                node.replaceWith(text);
                node = text;
            }
            this.nodes[$id$] = node;
        }"#
    }
    fn replace_placeholder(ptr: u32, len: u8, n: u16) {
        "{this.els = this.stack.splice(this.stack.length - $n$); let node = this.LoadChild($ptr$, $len$); node.replaceWith(...this.els);}"
    }
    fn load_template(tmpl_id: u16, index: u16, id: u32) {
        "{let node = this.templates[$tmpl_id$][$index$].cloneNode(true); this.nodes[$id$] = node; this.stack.push(node);}"
    }

    /*
    Binary protocol methods only!

    These methods let us support binary packing mutations for use on boundaries like desktop where we prefer to send
    binary data instead of JSON.

    We're using native types in a number of places
    */
    fn append_children_to_top(many: u16) {
        "{
        let root = this.stack[this.stack.length-many-1];
        this.els = this.stack.splice(this.stack.length-many);
        for (let k = 0; k < many; k++) {
            root.appendChild(this.els[k]);
        }
    }"
    }
    fn set_top_attribute(field: &str<u8, attr>, value: &str, ns: &str<u8, ns_cache>) {
        "{this.setAttributeInner(this.stack[this.stack.length-1], $field$, $value$, $ns$);}"
    }
    fn add_placeholder() {
        "{let node = document.createElement('pre'); node.hidden = true; this.stack.push(node);}"
    }
    fn create_element(element: &'static str<u8, el>) {
        "{this.stack.push(document.createElement($element$))}"
    }
    fn create_element_ns(element: &'static str<u8, el>, ns: &'static str<u8, namespace>) {
        "{this.stack.push(document.createElementNS($ns$, $element$))}"
    }
    fn add_templates(tmpl_id: u16, len: u16) {
        "{this.templates[$tmpl_id$] = this.stack.splice(this.stack.length-$len$);}"
    }
    fn foreign_event_listener(event: &str<u8, evt>, id: u32, bubbles: u8) {
        r#"
    bubbles = bubbles == 1;
    let node = this.nodes[id];
    if(node.listening){
        node.listening += 1;
    } else {
        node.listening = 1;
    }
    node.setAttribute('data-dioxus-id', `\${id}`);
    const event_name = $event$;

    // if this is a mounted listener, we send the event immediately
    if (event_name === "mounted") {
        window.ipc.postMessage(
            this.serializeIpcMessage("user_event", {
                name: event_name,
                element: id,
                data: null,
                bubbles,
            })
        );
    } else {
        this.listeners.create(event_name, node, bubbles, (event) => {
            this.handler(event, event_name, bubbles);
        });
    }"#
    }
    fn assign_id_ref(array: &[u8], id: u32) {
        "{this.nodes[$id$] = this.LoadChild($array$);}"
    }
    fn hydrate_text_ref(array: &[u8], value: &str, id: u32) {
        r#"{
        let node = this.LoadChild($array$);
        if (node.nodeType == node.TEXT_NODE) {
            node.textContent = value;
        } else {
            let text = document.createTextNode(value);
            node.replaceWith(text);
            node = text;
        }
        this.nodes[$id$] = node;
    }"#
    }
    fn replace_placeholder_ref(array: &[u8], n: u16) {
        "{this.els = this.stack.splice(this.stack.length - $n$); let node = this.LoadChild($array$); node.replaceWith(...this.els);}"
    }
}
