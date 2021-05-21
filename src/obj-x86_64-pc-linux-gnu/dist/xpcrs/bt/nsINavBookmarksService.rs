//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsINavBookmarksService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINavBookmarkObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean skipTags; */
                    Method {
                        name: "GetSkipTags",
                        params: &[Param { name: "aSkipTags", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onBeginUpdateBatch (); */
                    Method {
                        name: "OnBeginUpdateBatch",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onEndUpdateBatch (); */
                    Method {
                        name: "OnEndUpdateBatch",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onItemChanged (in long long aItemId, in ACString aProperty, in boolean aIsAnnotationProperty, in AUTF8String aNewValue, in PRTime aLastModified, in unsigned short aItemType, in long long aParentId, in ACString aGuid, in ACString aParentGuid, in AUTF8String aOldValue, in unsigned short aSource); */
                    Method {
                        name: "OnItemChanged",
                        params: &[Param { name: "aItemId", ty: "i64" }, Param { name: "aProperty", ty: "*const ::nsstring::nsACString" }, Param { name: "aIsAnnotationProperty", ty: "bool" }, Param { name: "aNewValue", ty: "*const ::nsstring::nsACString" }, Param { name: "aLastModified", ty: "PRTime" }, Param { name: "aItemType", ty: "u16" }, Param { name: "aParentId", ty: "i64" }, Param { name: "aGuid", ty: "*const ::nsstring::nsACString" }, Param { name: "aParentGuid", ty: "*const ::nsstring::nsACString" }, Param { name: "aOldValue", ty: "*const ::nsstring::nsACString" }, Param { name: "aSource", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onItemMoved (in long long aItemId, in long long aOldParentId, in long aOldIndex, in long long aNewParentId, in long aNewIndex, in unsigned short aItemType, in ACString aGuid, in ACString aOldParentGuid, in ACString aNewParentGuid, in unsigned short aSource, in AUTF8String aURI); */
                    Method {
                        name: "OnItemMoved",
                        params: &[Param { name: "aItemId", ty: "i64" }, Param { name: "aOldParentId", ty: "i64" }, Param { name: "aOldIndex", ty: "i32" }, Param { name: "aNewParentId", ty: "i64" }, Param { name: "aNewIndex", ty: "i32" }, Param { name: "aItemType", ty: "u16" }, Param { name: "aGuid", ty: "*const ::nsstring::nsACString" }, Param { name: "aOldParentGuid", ty: "*const ::nsstring::nsACString" }, Param { name: "aNewParentGuid", ty: "*const ::nsstring::nsACString" }, Param { name: "aSource", ty: "u16" }, Param { name: "aURI", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINavBookmarksService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long long placesRoot; */
                    Method {
                        name: "GetPlacesRoot",
                        params: &[Param { name: "aPlacesRoot", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long bookmarksMenuFolder; */
                    Method {
                        name: "GetBookmarksMenuFolder",
                        params: &[Param { name: "aBookmarksMenuFolder", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long tagsFolder; */
                    Method {
                        name: "GetTagsFolder",
                        params: &[Param { name: "aTagsFolder", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long toolbarFolder; */
                    Method {
                        name: "GetToolbarFolder",
                        params: &[Param { name: "aToolbarFolder", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long totalSyncChanges; */
                    Method {
                        name: "GetTotalSyncChanges",
                        params: &[Param { name: "aTotalSyncChanges", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] long long insertBookmark (in long long aParentId, in nsIURI aURI, in long aIndex, in AUTF8String aTitle, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
                    Method {
                        name: "InsertBookmark",
                        params: &[Param { name: "aParentId", ty: "i64" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aIndex", ty: "i32" }, Param { name: "aTitle", ty: "*const ::nsstring::nsACString" }, Param { name: "aGuid", ty: "*const ::nsstring::nsACString" }, Param { name: "aSource", ty: "u16" }, Param { name: "_retval", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void removeItem (in long long aItemId, [optional] in unsigned short aSource); */
                    Method {
                        name: "RemoveItem",
                        params: &[Param { name: "aItemId", ty: "i64" }, Param { name: "aSource", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] long long createFolder (in long long aParentFolder, in AUTF8String name, in long index, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
                    Method {
                        name: "CreateFolder",
                        params: &[Param { name: "aParentFolder", ty: "i64" }, Param { name: "name", ty: "*const ::nsstring::nsACString" }, Param { name: "index", ty: "i32" }, Param { name: "aGuid", ty: "*const ::nsstring::nsACString" }, Param { name: "aSource", ty: "u16" }, Param { name: "_retval", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setItemTitle (in long long aItemId, in AUTF8String aTitle, [optional] in unsigned short aSource); */
                    Method {
                        name: "SetItemTitle",
                        params: &[Param { name: "aItemId", ty: "i64" }, Param { name: "aTitle", ty: "*const ::nsstring::nsACString" }, Param { name: "aSource", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getItemTitle (in long long aItemId); */
                    Method {
                        name: "GetItemTitle",
                        params: &[Param { name: "aItemId", ty: "i64" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setItemLastModified (in long long aItemId, in PRTime aLastModified, [optional] in unsigned short aSource); */
                    Method {
                        name: "SetItemLastModified",
                        params: &[Param { name: "aItemId", ty: "i64" }, Param { name: "aLastModified", ty: "PRTime" }, Param { name: "aSource", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long long getFolderIdForItem (in long long aItemId); */
                    Method {
                        name: "GetFolderIdForItem",
                        params: &[Param { name: "aItemId", ty: "i64" }, Param { name: "_retval", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addObserver (in nsINavBookmarkObserver observer, [optional] in boolean ownsWeak); */
                    Method {
                        name: "AddObserver",
                        params: &[Param { name: "observer", ty: "*const nsINavBookmarkObserver" }, Param { name: "ownsWeak", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeObserver (in nsINavBookmarkObserver observer); */
                    Method {
                        name: "RemoveObserver",
                        params: &[Param { name: "observer", ty: "*const nsINavBookmarkObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<nsINavBookmarkObserver> getObservers (); */
                    Method {
                        name: "GetObservers",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsINavBookmarkObserver>>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

