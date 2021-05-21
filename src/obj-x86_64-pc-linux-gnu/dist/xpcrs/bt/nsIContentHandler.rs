//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIContentHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentHandler",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleContent (in string aContentType, in nsIInterfaceRequestor aWindowContext, in nsIRequest aRequest); */
                    Method {
                        name: "HandleContent",
                        params: &[Param { name: "aContentType", ty: "*const libc::c_char" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "aRequest", ty: "*const nsIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

