/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIScreenManager.idl
 */

#ifndef __gen_nsIScreenManager_h__
#define __gen_nsIScreenManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIScreen_h__
#include "nsIScreen.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIScreenManager */
#define NS_ISCREENMANAGER_IID_STR "e8a96e60-6b61-4a14-bacc-53891604b502"

#define NS_ISCREENMANAGER_IID \
  {0xe8a96e60, 0x6b61, 0x4a14, \
    { 0xba, 0xcc, 0x53, 0x89, 0x16, 0x04, 0xb5, 0x02 }}

class NS_NO_VTABLE nsIScreenManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCREENMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScreenManager;

  /* nsIScreen screenForRect (in long left, in long top, in long width, in long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScreenForRect(int32_t left, int32_t top, int32_t width, int32_t height, nsIScreen **_retval) = 0;

  /* readonly attribute nsIScreen primaryScreen; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrimaryScreen(nsIScreen **aPrimaryScreen) = 0;

  /* readonly attribute int64_t totalScreenPixels; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTotalScreenPixels(int64_t *aTotalScreenPixels) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScreenManager, NS_ISCREENMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCREENMANAGER \
  NS_IMETHOD ScreenForRect(int32_t left, int32_t top, int32_t width, int32_t height, nsIScreen **_retval) override; \
  NS_IMETHOD GetPrimaryScreen(nsIScreen **aPrimaryScreen) override; \
  NS_IMETHOD GetTotalScreenPixels(int64_t *aTotalScreenPixels) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCREENMANAGER \
  nsresult ScreenForRect(int32_t left, int32_t top, int32_t width, int32_t height, nsIScreen **_retval); \
  nsresult GetPrimaryScreen(nsIScreen **aPrimaryScreen); \
  nsresult GetTotalScreenPixels(int64_t *aTotalScreenPixels); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCREENMANAGER(_to) \
  NS_IMETHOD ScreenForRect(int32_t left, int32_t top, int32_t width, int32_t height, nsIScreen **_retval) override { return _to ScreenForRect(left, top, width, height, _retval); } \
  NS_IMETHOD GetPrimaryScreen(nsIScreen **aPrimaryScreen) override { return _to GetPrimaryScreen(aPrimaryScreen); } \
  NS_IMETHOD GetTotalScreenPixels(int64_t *aTotalScreenPixels) override { return _to GetTotalScreenPixels(aTotalScreenPixels); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCREENMANAGER(_to) \
  NS_IMETHOD ScreenForRect(int32_t left, int32_t top, int32_t width, int32_t height, nsIScreen **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScreenForRect(left, top, width, height, _retval); } \
  NS_IMETHOD GetPrimaryScreen(nsIScreen **aPrimaryScreen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryScreen(aPrimaryScreen); } \
  NS_IMETHOD GetTotalScreenPixels(int64_t *aTotalScreenPixels) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTotalScreenPixels(aTotalScreenPixels); } 


#endif /* __gen_nsIScreenManager_h__ */
