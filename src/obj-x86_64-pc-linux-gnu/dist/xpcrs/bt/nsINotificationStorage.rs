//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/notification/nsINotificationStorage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINotificationStorageCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handle (in AString id, in AString title, in AString dir, in AString lang, in AString body, in AString tag, in AString icon, in AString data, in AString behavior, in AString serviceWorkerRegistrationScope); */
                    Method {
                        name: "Handle",
                        params: &[Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "title", ty: "*const ::nsstring::nsAString" }, Param { name: "dir", ty: "*const ::nsstring::nsAString" }, Param { name: "lang", ty: "*const ::nsstring::nsAString" }, Param { name: "body", ty: "*const ::nsstring::nsAString" }, Param { name: "tag", ty: "*const ::nsstring::nsAString" }, Param { name: "icon", ty: "*const ::nsstring::nsAString" }, Param { name: "data", ty: "*const ::nsstring::nsAString" }, Param { name: "behavior", ty: "*const ::nsstring::nsAString" }, Param { name: "serviceWorkerRegistrationScope", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void done (); */
                    Method {
                        name: "Done",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINotificationStorage",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void put (in AString origin, in AString id, in AString title, in AString dir, in AString lang, in AString body, in AString tag, in AString icon, in AString alertName, in AString data, in AString behavior, in AString serviceWorkerRegistrationScope); */
                    Method {
                        name: "Put",
                        params: &[Param { name: "origin", ty: "*const ::nsstring::nsAString" }, Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "title", ty: "*const ::nsstring::nsAString" }, Param { name: "dir", ty: "*const ::nsstring::nsAString" }, Param { name: "lang", ty: "*const ::nsstring::nsAString" }, Param { name: "body", ty: "*const ::nsstring::nsAString" }, Param { name: "tag", ty: "*const ::nsstring::nsAString" }, Param { name: "icon", ty: "*const ::nsstring::nsAString" }, Param { name: "alertName", ty: "*const ::nsstring::nsAString" }, Param { name: "data", ty: "*const ::nsstring::nsAString" }, Param { name: "behavior", ty: "*const ::nsstring::nsAString" }, Param { name: "serviceWorkerRegistrationScope", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void get (in AString origin, in AString tag, in nsINotificationStorageCallback aCallback); */
                    Method {
                        name: "Get",
                        params: &[Param { name: "origin", ty: "*const ::nsstring::nsAString" }, Param { name: "tag", ty: "*const ::nsstring::nsAString" }, Param { name: "aCallback", ty: "*const nsINotificationStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getByID (in AString origin, in AString id, in nsINotificationStorageCallback aCallback); */
                    Method {
                        name: "GetByID",
                        params: &[Param { name: "origin", ty: "*const ::nsstring::nsAString" }, Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "aCallback", ty: "*const nsINotificationStorageCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void delete (in AString origin, in AString id); */
                    Method {
                        name: "Delete",
                        params: &[Param { name: "origin", ty: "*const ::nsstring::nsAString" }, Param { name: "id", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean canPut (in AString origin); */
                    Method {
                        name: "CanPut",
                        params: &[Param { name: "origin", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

