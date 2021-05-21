//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/mozapps/extensions/amIWebInstallPrompt.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "amIWebInstallPrompt",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void confirm (in Element aBrowser, in nsIURI aUri, in Array<nsIVariant> aInstalls); */
                    Method {
                        name: "Confirm",
                        params: &[Param { name: "aBrowser", ty: "*const libc::c_void" }, Param { name: "aUri", ty: "*const nsIURI" }, Param { name: "aInstalls", ty: "*const thin_vec::ThinVec<RefPtr<nsIVariant>>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

