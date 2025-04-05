use std::time::{self, Instant, SystemTime, UNIX_EPOCH};

use fastcomments_client::apis::public_api::GetCommentsPublicParams;
use fastcomments_core::sso::fastcomments_sso::FastCommentsSSO;

pub fn comments_params(tenant_id: String, url_id: String, sso_token: Option<String>) -> GetCommentsPublicParams {
    GetCommentsPublicParams {
        tenant_id,
        url_id,
        page: None,
        direction: None,
        sso: sso_token,
        skip: None,
        skip_children: None,
        limit: None,
        limit_children: None,
        count_children: None,
        last_gen_date: None,
        fetch_page_for_comment_id: None,
        include_config: None,
        count_all: None,
        includei10n: None,
        locale: None,
        modules: None,
        is_crawler: None,
        include_notification_count: None,
        as_tree: None,
        max_tree_depth: None,
        use_full_translation_ids: None,
        parent_id: None,
        search_text: None,
        hash_tags: None,
        user_id: None,
        custom_config_str: None,
        
    }
}