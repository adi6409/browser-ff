//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/cascade_bloom_filter/nsICascadeFilter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICascadeFilter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setFilterData (in Array<octet> data); */
                    Method {
                        name: "SetFilterData",
                        params: &[Param { name: "data", ty: "*const thin_vec::ThinVec<u8>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean has (in ACString key); */
                    Method {
                        name: "Has",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

