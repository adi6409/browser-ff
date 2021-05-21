//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIFilePicker.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFilePickerShownCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void done (in short aResult); */
                    Method {
                        name: "Done",
                        params: &[Param { name: "aResult", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFilePicker",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in mozIDOMWindowProxy parent, in AString title, in short mode); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "title", ty: "*const ::nsstring::nsAString" }, Param { name: "mode", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void appendFilters (in long filterMask); */
                    Method {
                        name: "AppendFilters",
                        params: &[Param { name: "filterMask", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void appendFilter (in AString title, in AString filter); */
                    Method {
                        name: "AppendFilter",
                        params: &[Param { name: "title", ty: "*const ::nsstring::nsAString" }, Param { name: "filter", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void appendRawFilter (in AString filter); */
                    Method {
                        name: "AppendRawFilter",
                        params: &[Param { name: "filter", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString defaultString; */
                    Method {
                        name: "GetDefaultString",
                        params: &[Param { name: "aDefaultString", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDefaultString",
                        params: &[Param { name: "aDefaultString", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString defaultExtension; */
                    Method {
                        name: "GetDefaultExtension",
                        params: &[Param { name: "aDefaultExtension", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDefaultExtension",
                        params: &[Param { name: "aDefaultExtension", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long filterIndex; */
                    Method {
                        name: "GetFilterIndex",
                        params: &[Param { name: "aFilterIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFilterIndex",
                        params: &[Param { name: "aFilterIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIFile displayDirectory; */
                    Method {
                        name: "GetDisplayDirectory",
                        params: &[Param { name: "aDisplayDirectory", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDisplayDirectory",
                        params: &[Param { name: "aDisplayDirectory", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString displaySpecialDirectory; */
                    Method {
                        name: "GetDisplaySpecialDirectory",
                        params: &[Param { name: "aDisplaySpecialDirectory", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDisplaySpecialDirectory",
                        params: &[Param { name: "aDisplaySpecialDirectory", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile file; */
                    Method {
                        name: "GetFile",
                        params: &[Param { name: "aFile", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI fileURL; */
                    Method {
                        name: "GetFileURL",
                        params: &[Param { name: "aFileURL", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator files; */
                    Method {
                        name: "GetFiles",
                        params: &[Param { name: "aFiles", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISupports domFileOrDirectory; */
                    Method {
                        name: "GetDomFileOrDirectory",
                        params: &[Param { name: "aDomFileOrDirectory", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator domFileOrDirectoryEnumerator; */
                    Method {
                        name: "GetDomFileOrDirectoryEnumerator",
                        params: &[Param { name: "aDomFileOrDirectoryEnumerator", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean addToRecentDocs; */
                    Method {
                        name: "GetAddToRecentDocs",
                        params: &[Param { name: "aAddToRecentDocs", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAddToRecentDocs",
                        params: &[Param { name: "aAddToRecentDocs", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void open (in nsIFilePickerShownCallback aFilePickerShownCallback); */
                    Method {
                        name: "Open",
                        params: &[Param { name: "aFilePickerShownCallback", ty: "*const nsIFilePickerShownCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute short mode; */
                    Method {
                        name: "GetMode",
                        params: &[Param { name: "aMode", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString okButtonLabel; */
                    Method {
                        name: "GetOkButtonLabel",
                        params: &[Param { name: "aOkButtonLabel", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOkButtonLabel",
                        params: &[Param { name: "aOkButtonLabel", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute short capture; */
                    Method {
                        name: "GetCapture",
                        params: &[Param { name: "aCapture", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCapture",
                        params: &[Param { name: "aCapture", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

