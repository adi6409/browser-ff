//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowserChromeFocus.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserChromeFocus",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void focusNextElement (in bool aForDocumentNavigation); */
                    Method {
                        name: "FocusNextElement",
                        params: &[Param { name: "aForDocumentNavigation", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void focusPrevElement (in bool aForDocumentNavigation); */
                    Method {
                        name: "FocusPrevElement",
                        params: &[Param { name: "aForDocumentNavigation", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

