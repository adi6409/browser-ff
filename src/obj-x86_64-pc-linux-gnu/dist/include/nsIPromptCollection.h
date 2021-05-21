/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIPromptCollection.idl
 */

#ifndef __gen_nsIPromptCollection_h__
#define __gen_nsIPromptCollection_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIPromptCollection */
#define NS_IPROMPTCOLLECTION_IID_STR "7913837c-9623-11ea-bb37-0242ac130002"

#define NS_IPROMPTCOLLECTION_IID \
  {0x7913837c, 0x9623, 0x11ea, \
    { 0xbb, 0x37, 0x02, 0x42, 0xac, 0x13, 0x00, 0x02 }}

class NS_NO_VTABLE nsIPromptCollection : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROMPTCOLLECTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPromptCollection;

  /* Promise asyncBeforeUnloadCheck (in BrowsingContext aBrowsingContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncBeforeUnloadCheck(mozilla::dom::BrowsingContext *aBrowsingContext, ::mozilla::dom::Promise * * _retval) = 0;

  /* boolean confirmRepost (in BrowsingContext aBrowsingContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConfirmRepost(mozilla::dom::BrowsingContext *aBrowsingContext, bool *_retval) = 0;

  /* boolean confirmFolderUpload (in BrowsingContext aBrowsingContext, in AString aDirectoryName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConfirmFolderUpload(mozilla::dom::BrowsingContext *aBrowsingContext, const nsAString& aDirectoryName, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPromptCollection, NS_IPROMPTCOLLECTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROMPTCOLLECTION \
  NS_IMETHOD AsyncBeforeUnloadCheck(mozilla::dom::BrowsingContext *aBrowsingContext, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD ConfirmRepost(mozilla::dom::BrowsingContext *aBrowsingContext, bool *_retval) override; \
  NS_IMETHOD ConfirmFolderUpload(mozilla::dom::BrowsingContext *aBrowsingContext, const nsAString& aDirectoryName, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROMPTCOLLECTION \
  nsresult AsyncBeforeUnloadCheck(mozilla::dom::BrowsingContext *aBrowsingContext, ::mozilla::dom::Promise * * _retval); \
  nsresult ConfirmRepost(mozilla::dom::BrowsingContext *aBrowsingContext, bool *_retval); \
  nsresult ConfirmFolderUpload(mozilla::dom::BrowsingContext *aBrowsingContext, const nsAString& aDirectoryName, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROMPTCOLLECTION(_to) \
  NS_IMETHOD AsyncBeforeUnloadCheck(mozilla::dom::BrowsingContext *aBrowsingContext, ::mozilla::dom::Promise * * _retval) override { return _to AsyncBeforeUnloadCheck(aBrowsingContext, _retval); } \
  NS_IMETHOD ConfirmRepost(mozilla::dom::BrowsingContext *aBrowsingContext, bool *_retval) override { return _to ConfirmRepost(aBrowsingContext, _retval); } \
  NS_IMETHOD ConfirmFolderUpload(mozilla::dom::BrowsingContext *aBrowsingContext, const nsAString& aDirectoryName, bool *_retval) override { return _to ConfirmFolderUpload(aBrowsingContext, aDirectoryName, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROMPTCOLLECTION(_to) \
  NS_IMETHOD AsyncBeforeUnloadCheck(mozilla::dom::BrowsingContext *aBrowsingContext, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncBeforeUnloadCheck(aBrowsingContext, _retval); } \
  NS_IMETHOD ConfirmRepost(mozilla::dom::BrowsingContext *aBrowsingContext, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConfirmRepost(aBrowsingContext, _retval); } \
  NS_IMETHOD ConfirmFolderUpload(mozilla::dom::BrowsingContext *aBrowsingContext, const nsAString& aDirectoryName, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConfirmFolderUpload(aBrowsingContext, aDirectoryName, _retval); } 


#endif /* __gen_nsIPromptCollection_h__ */
