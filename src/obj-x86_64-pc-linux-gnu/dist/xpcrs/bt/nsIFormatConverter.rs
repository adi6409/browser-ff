//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIFormatConverter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFormatConverter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* Array<ACString> getInputDataFlavors (); */
                    Method {
                        name: "GetInputDataFlavors",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> getOutputDataFlavors (); */
                    Method {
                        name: "GetOutputDataFlavors",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean canConvert (in string aFromDataFlavor, in string aToDataFlavor); */
                    Method {
                        name: "CanConvert",
                        params: &[Param { name: "aFromDataFlavor", ty: "*const libc::c_char" }, Param { name: "aToDataFlavor", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void convert (in string aFromDataFlavor, in nsISupports aFromData, in string aToDataFlavor, out nsISupports aToData); */
                    Method {
                        name: "Convert",
                        params: &[Param { name: "aFromDataFlavor", ty: "*const libc::c_char" }, Param { name: "aFromData", ty: "*const nsISupports" }, Param { name: "aToDataFlavor", ty: "*const libc::c_char" }, Param { name: "aToData", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

