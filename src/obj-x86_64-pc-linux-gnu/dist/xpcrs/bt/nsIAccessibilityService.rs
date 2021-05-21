//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibilityService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibilityService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIAccessible getApplicationAccessible (); */
                    Method {
                        name: "GetApplicationAccessible",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessible getAccessibleFor (in Node aNode); */
                    Method {
                        name: "GetAccessibleFor",
                        params: &[Param { name: "aNode", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getStringRole (in unsigned long aRole); */
                    Method {
                        name: "GetStringRole",
                        params: &[Param { name: "aRole", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports getStringStates (in unsigned long aStates, in unsigned long aExtraStates); */
                    Method {
                        name: "GetStringStates",
                        params: &[Param { name: "aStates", ty: "u32" }, Param { name: "aExtraStates", ty: "u32" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getStringEventType (in unsigned long aEventType); */
                    Method {
                        name: "GetStringEventType",
                        params: &[Param { name: "aEventType", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getStringRelationType (in unsigned long aRelationType); */
                    Method {
                        name: "GetStringRelationType",
                        params: &[Param { name: "aRelationType", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessible getAccessibleFromCache (in Node aNode); */
                    Method {
                        name: "GetAccessibleFromCache",
                        params: &[Param { name: "aNode", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessiblePivot createAccessiblePivot (in nsIAccessible aRoot); */
                    Method {
                        name: "CreateAccessiblePivot",
                        params: &[Param { name: "aRoot", ty: "*const nsIAccessible" }, Param { name: "_retval", ty: "*mut*const nsIAccessiblePivot" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setLogging (in ACString aModules); */
                    Method {
                        name: "SetLogging",
                        params: &[Param { name: "aModules", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isLogged (in AString aModule); */
                    Method {
                        name: "IsLogged",
                        params: &[Param { name: "aModule", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getConsumers (); */
                    Method {
                        name: "GetConsumers",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

