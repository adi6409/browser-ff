//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsITaggingService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITaggingService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void tagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource); */
                    Method {
                        name: "TagURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aTags", ty: "*const nsIVariant" }, Param { name: "aSource", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void untagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource); */
                    Method {
                        name: "UntagURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aTags", ty: "*const nsIVariant" }, Param { name: "aSource", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<AString> getTagsForURI (in nsIURI aURI); */
                    Method {
                        name: "GetTagsForURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsString>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

