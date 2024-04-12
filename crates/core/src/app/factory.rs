use ngyn_shared::{NgynContext, NgynEngine, NgynModule, NgynResponse};

/// The `NgynFactory` struct is used to create instances of `NgynEngine`.
pub struct NgynFactory<Application: NgynEngine> {
    /// this is just a placeholder and would prolly not be used
    _app: Application,
}

impl<Application: NgynEngine> NgynFactory<Application> {
    #[allow(dead_code)]
    /// The `create` method takes a generic parameter `AppModule` that implements the `NgynModule` trait.
    /// It returns an instance of `NgynEngine`.
    ///
    /// ### Example
    ///
    /// ```
    /// use ngyn::{platforms::NgynApplication, prelude::*};
    ///
    /// #[module]
    /// pub struct YourAppModule;
    ///
    /// let server = NgynFactory::<NgynApplication>::create::<YourAppModule>();
    /// ```
    pub fn create<AppModule: NgynModule>() -> Application {
        let mut module = AppModule::new(vec![]);
        let mut server = Application::new();
        for controller in module.get_controllers() {
            for (path, http_method, handler) in controller.routes() {
                server.route(
                    path.as_str(),
                    http_method.into(),
                    Box::new({
                        let controller = controller.clone();
                        move |cx: &mut NgynContext, res: &mut NgynResponse| {
                            let _ = controller.handle(&handler, cx, res);
                        }
                    }),
                );
            }
        }
        server
    }
}
