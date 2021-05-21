//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageRow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageRow",
            base: Some("mozIStorageValueArray"),
            methods: Ok(&[
                    /* nsIVariant getResultByIndex (in unsigned long aIndex); */
                    Method {
                        name: "GetResultByIndex",
                        params: &[Param { name: "aIndex", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIVariant getResultByName (in AUTF8String aName); */
                    Method {
                        name: "GetResultByName",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

