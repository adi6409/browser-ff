//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/simpledb/nsISDBCallbacks.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISDBCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onComplete (in nsISDBRequest aRequest); */
                    Method {
                        name: "OnComplete",
                        params: &[Param { name: "aRequest", ty: "*const nsISDBRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISDBCloseCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onClose (in nsISDBConnection aConnection); */
                    Method {
                        name: "OnClose",
                        params: &[Param { name: "aConnection", ty: "*const nsISDBConnection" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

