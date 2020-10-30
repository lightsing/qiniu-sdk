use qiniu_sys::Qiniu_Servend_Init;


struct Client;

impl Client {
    fn new_server(access_key: String, secret_key: String) -> Self {
        Qiniu_Servend_Init(-1);

        Client { }
    }
}
