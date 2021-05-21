//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/sidebar/nsIWebProtocolHandlerRegistrar.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebProtocolHandlerRegistrar",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void registerProtocolHandler (in AString protocol, in nsIURI uri, in AString title, in nsIURI documentURI, in nsISupports windowOrBrowser); */
                    Method {
                        name: "RegisterProtocolHandler",
                        params: &[Param { name: "protocol", ty: "*const ::nsstring::nsAString" }, Param { name: "uri", ty: "*const nsIURI" }, Param { name: "title", ty: "*const ::nsstring::nsAString" }, Param { name: "documentURI", ty: "*const nsIURI" }, Param { name: "windowOrBrowser", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeProtocolHandler (in AString protocol, in AString uri); */
                    Method {
                        name: "RemoveProtocolHandler",
                        params: &[Param { name: "protocol", ty: "*const ::nsstring::nsAString" }, Param { name: "uri", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

