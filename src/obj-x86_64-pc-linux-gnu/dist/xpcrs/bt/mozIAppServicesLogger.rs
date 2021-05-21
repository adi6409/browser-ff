//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/interfaces/mozIAppServicesLogger.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIAppServicesLogger",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void register (in AString target, in mozIServicesLogSink logger); */
                    Method {
                        name: "Register",
                        params: &[Param { name: "target", ty: "*const ::nsstring::nsAString" }, Param { name: "logger", ty: "*const mozIServicesLogSink" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

