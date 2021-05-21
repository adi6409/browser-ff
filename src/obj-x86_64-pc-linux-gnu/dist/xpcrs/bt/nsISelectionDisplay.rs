//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISelectionDisplay.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISelectionDisplay",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setSelectionFlags (in short toggle); */
                    Method {
                        name: "SetSelectionFlags",
                        params: &[Param { name: "toggle", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* short getSelectionFlags (); */
                    Method {
                        name: "GetSelectionFlags",
                        params: &[Param { name: "_retval", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

