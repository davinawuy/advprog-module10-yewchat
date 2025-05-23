use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    //log::debug!("got input: {:?}", input.value());
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
  html! {
        <div class="flex w-screen min-h-screen bg-gray-900 text-white overflow-hidden">
            // Sidebar
            <aside class="flex-none w-64 bg-gray-800 p-4 border-r border-gray-700 overflow-y-auto">
                <h2 class="text-lg font-semibold text-violet-400 mb-4">{"Online Users"}</h2>
                {
                    for self.users.iter().map(|u| html! {
                        <div class="flex items-center gap-4 bg-gray-700 p-3 rounded-lg mb-3 shadow-sm">
                            <img class="w-10 h-10 rounded-full" src={u.avatar.clone()} alt="avatar" />
                            <div class="text-sm">
                                <p class="font-medium">{ &u.name }</p>
                                <p class="text-gray-400 text-xs">{"Hi there!"}</p>
                            </div>
                        </div>
                    })
                }
            </aside>

            // Chat Area
            <main class="flex-1 flex flex-col">
                // Header
                <header class="h-16 px-6 flex items-center border-b border-gray-700">
                    <h1 class="text-xl font-bold text-violet-300">{"💬 Live Chat"}</h1>
                </header>

                // Messages
                <section class="flex-1 overflow-y-auto px-6 py-4 space-y-4">
                    {
                        for self.messages.iter().map(|m| {
                            let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                            html! {
                                <div class="flex items-start gap-3 bg-gray-800 rounded-lg p-4 shadow-sm max-w-xl">
                                    <img class="w-8 h-8 rounded-full" src={user.avatar.clone()} alt="avatar"/>
                                    <div>
                                        <p class="text-sm font-semibold text-violet-400">{ &m.from }</p>
                                        <div class="text-sm text-gray-300 mt-1 break-words">
                                            {
                                                if m.message.ends_with(".gif") {
                                                    html! { <img src={m.message.clone()} class="rounded-lg mt-2 max-w-xs"/> }
                                                } else {
                                                    html! { &m.message }
                                                }
                                            }
                                        </div>
                                    </div>
                                </div>
                            }
                        })
                    }
                </section>

                // Message Input
                <footer class="h-16 px-6 flex items-center border-t border-gray-700 bg-gray-800">
                    <input
                        ref={self.chat_input.clone()}
                        type="text"
                        name="message"
                        placeholder="Type your message..."
                        required=true
                        class="flex-1 px-4 py-2 mr-3 rounded-full bg-gray-700 text-white placeholder-gray-400
                               focus:outline-none focus:ring-2 focus:ring-violet-500 transition-all"
                    />
                    <button
                        onclick={submit}
                        class="w-11 h-11 rounded-full bg-violet-600 hover:bg-violet-700 transition
                               flex justify-center items-center focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500"
                    >
                        <svg viewBox="0 0 24 24" fill="none" class="w-5 h-5 text-white">
                            <path d="M2 21l21-9L2 3v7l15 2-15 2v7z" fill="currentColor"/>
                        </svg>
                    </button>
                </footer>
            </main>
        </div>
    }
    }
}