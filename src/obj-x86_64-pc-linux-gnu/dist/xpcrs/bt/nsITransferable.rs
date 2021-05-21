//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsITransferable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFlavorDataProvider",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getFlavorData (in nsITransferable aTransferable, in string aFlavor, out nsISupports aData); */
                    Method {
                        name: "GetFlavorData",
                        params: &[Param { name: "aTransferable", ty: "*const nsITransferable" }, Param { name: "aFlavor", ty: "*const libc::c_char" }, Param { name: "aData", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsITransferable",
            base: Some("nsISupports"),
            methods: Err("nostdcall is unsupported"),
        },

        ]; D}

