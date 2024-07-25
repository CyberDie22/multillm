pub mod request {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChatCompletionRequest {
        pub model: String,
        pub frequency_penalty: Option<f32>,
        pub presence_penalty: Option<f32>,
        pub n: Option<i32>,
        pub response_format: Option<ChatResponseFormat>,
        pub seed: Option<i32>,
        pub stop: Option<String>,
        pub stream: Option<bool>,
        pub temperature: Option<f32>,
        pub top_p: Option<f32>,
        pub user: Option<String>,
        // TODO: logit_bias, logprobs, tools, tool_choice, functions (dep), function_call (dep)
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChatResponseFormat {
        #[serde(rename = "type")]
        pub type_: String,
    }
}

pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct ChatCompletionResponse {
        pub id: String,
        pub created: i32,
        pub model: String,
        pub object: String,
        pub usage: TokenUsage
    }

    #[derive(Serialize, Deserialize)]
    pub struct ChatCompletionChoices {
        pub finish_reason: String,
        pub index: i32,
        pub content: String,
        pub role: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ToolCall {
        pub id: String,
        #[serde(rename = "type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct FunctionToolCall {
        pub name: String,
        pub arguments: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct TokenUsage {
        pub completion_tokens: i32,
        pub prompt_tokens: i32,
        pub total_tokens: i32
    }
}