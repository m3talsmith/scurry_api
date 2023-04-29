use postgrest::Postgrest;

pub struct Supabase {
    pub api_url: String,
    pub api_key: String,
    pub client: Postgrest
}