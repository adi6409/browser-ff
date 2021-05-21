//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIMessageManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMessageSender",
            base: Some("nsISupports"),
            methods: Ok(&[
                    ]),
        },

        Interface {
            name: "nsIInProcessContentFrameMessageManager",
            base: Some("nsIMessageSender"),
            methods: Ok(&[
                    /* [notxpcom] nsIContent getOwnerContent (); */
                    Method {
                        name: "GetOwnerContent",
                        params: &[],
                        ret: "*const nsIContent",
                    },

                    ]),
        },

        ]; D}

