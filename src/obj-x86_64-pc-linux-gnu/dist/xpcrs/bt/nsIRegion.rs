//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/modules/nsIRegion.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRegion",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString current; */
                    Method {
                        name: "GetCurrent",
                        params: &[Param { name: "aCurrent", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString home; */
                    Method {
                        name: "GetHome",
                        params: &[Param { name: "aHome", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

