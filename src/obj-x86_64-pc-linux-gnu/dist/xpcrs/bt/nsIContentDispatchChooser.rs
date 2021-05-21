//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIContentDispatchChooser.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentDispatchChooser",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleURI (in nsIHandlerInfo aHandler, in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in BrowsingContext aBrowsingContext); */
                    Method {
                        name: "HandleURI",
                        params: &[Param { name: "aHandler", ty: "*const nsIHandlerInfo" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aBrowsingContext", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

