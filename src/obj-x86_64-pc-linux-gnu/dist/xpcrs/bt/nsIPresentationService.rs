//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationServiceCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void notifySuccess (in AString url); */
                    Method {
                        name: "NotifySuccess",
                        params: &[Param { name: "url", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyError (in nsresult error); */
                    Method {
                        name: "NotifyError",
                        params: &[Param { name: "error", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationService",
            base: Some("nsISupports"),
            methods: Err("native type const nsTArray<nsString> unsupported"),
        },

        ]; D}

