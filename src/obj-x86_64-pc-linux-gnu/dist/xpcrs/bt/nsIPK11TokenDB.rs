//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPK11TokenDB.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPK11TokenDB",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIPK11Token getInternalKeyToken (); */
                    Method {
                        name: "GetInternalKeyToken",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIPK11Token" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

