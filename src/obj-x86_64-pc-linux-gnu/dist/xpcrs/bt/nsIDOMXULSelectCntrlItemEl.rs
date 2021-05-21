//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULSelectCntrlItemEl.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULSelectControlItemElement",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute boolean disabled; */
                    Method {
                        name: "GetDisabled",
                        params: &[Param { name: "aDisabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDisabled",
                        params: &[Param { name: "aDisabled", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString crop; */
                    Method {
                        name: "GetCrop",
                        params: &[Param { name: "aCrop", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCrop",
                        params: &[Param { name: "aCrop", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString image; */
                    Method {
                        name: "GetImage",
                        params: &[Param { name: "aImage", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetImage",
                        params: &[Param { name: "aImage", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString label; */
                    Method {
                        name: "GetLabel",
                        params: &[Param { name: "aLabel", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLabel",
                        params: &[Param { name: "aLabel", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString accessKey; */
                    Method {
                        name: "GetAccessKey",
                        params: &[Param { name: "aAccessKey", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAccessKey",
                        params: &[Param { name: "aAccessKey", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString command; */
                    Method {
                        name: "GetCommand",
                        params: &[Param { name: "aCommand", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCommand",
                        params: &[Param { name: "aCommand", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetValue",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean selected; */
                    Method {
                        name: "GetSelected",
                        params: &[Param { name: "aSelected", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Element control; */
                    Method {
                        name: "GetControl",
                        params: &[Param { name: "aControl", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

