/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIDroppedLinkHandler.idl
 */

#ifndef __gen_nsIDroppedLinkHandler_h__
#define __gen_nsIDroppedLinkHandler_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIPrincipal_h__
#include "nsIPrincipal.h"
#endif

#ifndef __gen_nsIContentSecurityPolicy_h__
#include "nsIContentSecurityPolicy.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class DragEvent; /* webidl DragEvent */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class DataTransfer; /* webidl DataTransfer */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDroppedLinkItem */
#define NS_IDROPPEDLINKITEM_IID_STR "69e14f91-2e09-4ca6-a511-a715c99a2804"

#define NS_IDROPPEDLINKITEM_IID \
  {0x69e14f91, 0x2e09, 0x4ca6, \
    { 0xa5, 0x11, 0xa7, 0x15, 0xc9, 0x9a, 0x28, 0x04 }}

class NS_NO_VTABLE nsIDroppedLinkItem : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDROPPEDLINKITEM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDroppedLinkItem;

  /* readonly attribute AString url; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUrl(nsAString& aUrl) = 0;

  /* readonly attribute AString name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsAString& aName) = 0;

  /* readonly attribute AString type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(nsAString& aType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDroppedLinkItem, NS_IDROPPEDLINKITEM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDROPPEDLINKITEM \
  NS_IMETHOD GetUrl(nsAString& aUrl) override; \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD GetType(nsAString& aType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDROPPEDLINKITEM \
  nsresult GetUrl(nsAString& aUrl); \
  nsresult GetName(nsAString& aName); \
  nsresult GetType(nsAString& aType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDROPPEDLINKITEM(_to) \
  NS_IMETHOD GetUrl(nsAString& aUrl) override { return _to GetUrl(aUrl); } \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetType(nsAString& aType) override { return _to GetType(aType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDROPPEDLINKITEM(_to) \
  NS_IMETHOD GetUrl(nsAString& aUrl) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUrl(aUrl); } \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetType(nsAString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } 


/* starting interface:    nsIDroppedLinkHandler */
#define NS_IDROPPEDLINKHANDLER_IID_STR "21b5c25a-28a9-47bd-8431-fa9116305ded"

#define NS_IDROPPEDLINKHANDLER_IID \
  {0x21b5c25a, 0x28a9, 0x47bd, \
    { 0x84, 0x31, 0xfa, 0x91, 0x16, 0x30, 0x5d, 0xed }}

class NS_NO_VTABLE nsIDroppedLinkHandler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDROPPEDLINKHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDroppedLinkHandler;

  /* boolean canDropLink (in DragEvent aEvent, in boolean aAllowSameDocument); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanDropLink(mozilla::dom::DragEvent *aEvent, bool aAllowSameDocument, bool *_retval) = 0;

  /* AString dropLink (in DragEvent aEvent, out AString aName, [optional] in boolean aDisallowInherit); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DropLink(mozilla::dom::DragEvent *aEvent, nsAString& aName, bool aDisallowInherit, nsAString& _retval) = 0;

  /* Array<nsIDroppedLinkItem> dropLinks (in DragEvent aEvent, [optional] in boolean aDisallowInherit); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DropLinks(mozilla::dom::DragEvent *aEvent, bool aDisallowInherit, nsTArray<RefPtr<nsIDroppedLinkItem>>& _retval) = 0;

  /* void validateURIsForDrop (in DragEvent aEvent, in Array<AString> aURIs, [optional] in boolean aDisallowInherit); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ValidateURIsForDrop(mozilla::dom::DragEvent *aEvent, const nsTArray<nsString >& aURIs, bool aDisallowInherit) = 0;

  /* Array<nsIDroppedLinkItem> queryLinks (in DataTransfer aDataTransfer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD QueryLinks(mozilla::dom::DataTransfer *aDataTransfer, nsTArray<RefPtr<nsIDroppedLinkItem>>& _retval) = 0;

  /* nsIPrincipal getTriggeringPrincipal (in DragEvent aEvent); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTriggeringPrincipal(mozilla::dom::DragEvent *aEvent, nsIPrincipal **_retval) = 0;

  /* nsIContentSecurityPolicy getCSP (in DragEvent aEvent); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCSP(mozilla::dom::DragEvent *aEvent, nsIContentSecurityPolicy **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDroppedLinkHandler, NS_IDROPPEDLINKHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDROPPEDLINKHANDLER \
  NS_IMETHOD CanDropLink(mozilla::dom::DragEvent *aEvent, bool aAllowSameDocument, bool *_retval) override; \
  NS_IMETHOD DropLink(mozilla::dom::DragEvent *aEvent, nsAString& aName, bool aDisallowInherit, nsAString& _retval) override; \
  NS_IMETHOD DropLinks(mozilla::dom::DragEvent *aEvent, bool aDisallowInherit, nsTArray<RefPtr<nsIDroppedLinkItem>>& _retval) override; \
  NS_IMETHOD ValidateURIsForDrop(mozilla::dom::DragEvent *aEvent, const nsTArray<nsString >& aURIs, bool aDisallowInherit) override; \
  NS_IMETHOD QueryLinks(mozilla::dom::DataTransfer *aDataTransfer, nsTArray<RefPtr<nsIDroppedLinkItem>>& _retval) override; \
  NS_IMETHOD GetTriggeringPrincipal(mozilla::dom::DragEvent *aEvent, nsIPrincipal **_retval) override; \
  NS_IMETHOD GetCSP(mozilla::dom::DragEvent *aEvent, nsIContentSecurityPolicy **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDROPPEDLINKHANDLER \
  nsresult CanDropLink(mozilla::dom::DragEvent *aEvent, bool aAllowSameDocument, bool *_retval); \
  nsresult DropLink(mozilla::dom::DragEvent *aEvent, nsAString& aName, bool aDisallowInherit, nsAString& _retval); \
  nsresult DropLinks(mozilla::dom::DragEvent *aEvent, bool aDisallowInherit, nsTArray<RefPtr<nsIDroppedLinkItem>>& _retval); \
  nsresult ValidateURIsForDrop(mozilla::dom::DragEvent *aEvent, const nsTArray<nsString >& aURIs, bool aDisallowInherit); \
  nsresult QueryLinks(mozilla::dom::DataTransfer *aDataTransfer, nsTArray<RefPtr<nsIDroppedLinkItem>>& _retval); \
  nsresult GetTriggeringPrincipal(mozilla::dom::DragEvent *aEvent, nsIPrincipal **_retval); \
  nsresult GetCSP(mozilla::dom::DragEvent *aEvent, nsIContentSecurityPolicy **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDROPPEDLINKHANDLER(_to) \
  NS_IMETHOD CanDropLink(mozilla::dom::DragEvent *aEvent, bool aAllowSameDocument, bool *_retval) override { return _to CanDropLink(aEvent, aAllowSameDocument, _retval); } \
  NS_IMETHOD DropLink(mozilla::dom::DragEvent *aEvent, nsAString& aName, bool aDisallowInherit, nsAString& _retval) override { return _to DropLink(aEvent, aName, aDisallowInherit, _retval); } \
  NS_IMETHOD DropLinks(mozilla::dom::DragEvent *aEvent, bool aDisallowInherit, nsTArray<RefPtr<nsIDroppedLinkItem>>& _retval) override { return _to DropLinks(aEvent, aDisallowInherit, _retval); } \
  NS_IMETHOD ValidateURIsForDrop(mozilla::dom::DragEvent *aEvent, const nsTArray<nsString >& aURIs, bool aDisallowInherit) override { return _to ValidateURIsForDrop(aEvent, aURIs, aDisallowInherit); } \
  NS_IMETHOD QueryLinks(mozilla::dom::DataTransfer *aDataTransfer, nsTArray<RefPtr<nsIDroppedLinkItem>>& _retval) override { return _to QueryLinks(aDataTransfer, _retval); } \
  NS_IMETHOD GetTriggeringPrincipal(mozilla::dom::DragEvent *aEvent, nsIPrincipal **_retval) override { return _to GetTriggeringPrincipal(aEvent, _retval); } \
  NS_IMETHOD GetCSP(mozilla::dom::DragEvent *aEvent, nsIContentSecurityPolicy **_retval) override { return _to GetCSP(aEvent, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDROPPEDLINKHANDLER(_to) \
  NS_IMETHOD CanDropLink(mozilla::dom::DragEvent *aEvent, bool aAllowSameDocument, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanDropLink(aEvent, aAllowSameDocument, _retval); } \
  NS_IMETHOD DropLink(mozilla::dom::DragEvent *aEvent, nsAString& aName, bool aDisallowInherit, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DropLink(aEvent, aName, aDisallowInherit, _retval); } \
  NS_IMETHOD DropLinks(mozilla::dom::DragEvent *aEvent, bool aDisallowInherit, nsTArray<RefPtr<nsIDroppedLinkItem>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DropLinks(aEvent, aDisallowInherit, _retval); } \
  NS_IMETHOD ValidateURIsForDrop(mozilla::dom::DragEvent *aEvent, const nsTArray<nsString >& aURIs, bool aDisallowInherit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ValidateURIsForDrop(aEvent, aURIs, aDisallowInherit); } \
  NS_IMETHOD QueryLinks(mozilla::dom::DataTransfer *aDataTransfer, nsTArray<RefPtr<nsIDroppedLinkItem>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->QueryLinks(aDataTransfer, _retval); } \
  NS_IMETHOD GetTriggeringPrincipal(mozilla::dom::DragEvent *aEvent, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTriggeringPrincipal(aEvent, _retval); } \
  NS_IMETHOD GetCSP(mozilla::dom::DragEvent *aEvent, nsIContentSecurityPolicy **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCSP(aEvent, _retval); } 


#endif /* __gen_nsIDroppedLinkHandler_h__ */
