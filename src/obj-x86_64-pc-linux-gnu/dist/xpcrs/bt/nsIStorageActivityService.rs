//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/storage/nsIStorageActivityService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStorageActivityService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIArray getActiveOrigins (in PRTime from, in PRTime to); */
                    Method {
                        name: "GetActiveOrigins",
                        params: &[Param { name: "from", ty: "PRTime" }, Param { name: "to", ty: "PRTime" }, Param { name: "_retval", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void moveOriginInTime (in nsIPrincipal origin, in PRTime when); */
                    Method {
                        name: "MoveOriginInTime",
                        params: &[Param { name: "origin", ty: "*const nsIPrincipal" }, Param { name: "when", ty: "PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void testOnlyReset (); */
                    Method {
                        name: "TestOnlyReset",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

