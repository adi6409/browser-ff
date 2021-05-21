//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIClipboardOwner.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClipboardOwner",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void LosingOwnership (in nsITransferable aTransferable); */
                    Method {
                        name: "LosingOwnership",
                        params: &[Param { name: "aTransferable", ty: "*const nsITransferable" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

