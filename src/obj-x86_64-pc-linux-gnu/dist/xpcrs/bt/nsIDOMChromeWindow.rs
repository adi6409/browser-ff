//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIDOMChromeWindow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMChromeWindow",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [noscript] readonly attribute nsIBrowserDOMWindow browserDOMWindow; */
                    Method {
                        name: "GetBrowserDOMWindow",
                        params: &[Param { name: "aBrowserDOMWindow", ty: "*mut*const nsIBrowserDOMWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

