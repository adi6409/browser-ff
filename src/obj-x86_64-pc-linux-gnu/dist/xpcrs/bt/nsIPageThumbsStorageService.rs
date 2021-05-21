//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/thumbnails/nsIPageThumbsStorageService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPageThumbsStorageService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* AString getLeafNameForURL (in AString aURL); */
                    Method {
                        name: "GetLeafNameForURL",
                        params: &[Param { name: "aURL", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString path; */
                    Method {
                        name: "GetPath",
                        params: &[Param { name: "aPath", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getFilePathForURL (in AString aURL); */
                    Method {
                        name: "GetFilePathForURL",
                        params: &[Param { name: "aURL", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

