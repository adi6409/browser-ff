/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/shell/nsIShellService.idl
 */

#ifndef __gen_nsIShellService_h__
#define __gen_nsIShellService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIShellService */
#define NS_ISHELLSERVICE_IID_STR "2d1a95e4-5bd8-4eeb-b0a8-c1455fd2a357"

#define NS_ISHELLSERVICE_IID \
  {0x2d1a95e4, 0x5bd8, 0x4eeb, \
    { 0xb0, 0xa8, 0xc1, 0x45, 0x5f, 0xd2, 0xa3, 0x57 }}

class NS_NO_VTABLE nsIShellService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISHELLSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIShellService;

  /* boolean isDefaultBrowser ([optional] in boolean aForAllTypes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsDefaultBrowser(bool aForAllTypes, bool *_retval) = 0;

  /* void setDefaultBrowser (in boolean aClaimAllTypes, in boolean aForAllUsers); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefaultBrowser(bool aClaimAllTypes, bool aForAllUsers) = 0;

  enum {
    BACKGROUND_TILE = 1,
    BACKGROUND_STRETCH = 2,
    BACKGROUND_CENTER = 3,
    BACKGROUND_FILL = 4,
    BACKGROUND_FIT = 5,
    BACKGROUND_SPAN = 6
  };

  /* void setDesktopBackground (in Element aElement, in long aPosition, in ACString aImageName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDesktopBackground(mozilla::dom::Element *aElement, int32_t aPosition, const nsACString& aImageName) = 0;

  /* attribute unsigned long desktopBackgroundColor; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDesktopBackgroundColor(uint32_t *aDesktopBackgroundColor) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDesktopBackgroundColor(uint32_t aDesktopBackgroundColor) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIShellService, NS_ISHELLSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISHELLSERVICE \
  NS_IMETHOD IsDefaultBrowser(bool aForAllTypes, bool *_retval) override; \
  NS_IMETHOD SetDefaultBrowser(bool aClaimAllTypes, bool aForAllUsers) override; \
  NS_IMETHOD SetDesktopBackground(mozilla::dom::Element *aElement, int32_t aPosition, const nsACString& aImageName) override; \
  NS_IMETHOD GetDesktopBackgroundColor(uint32_t *aDesktopBackgroundColor) override; \
  NS_IMETHOD SetDesktopBackgroundColor(uint32_t aDesktopBackgroundColor) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISHELLSERVICE \
  nsresult IsDefaultBrowser(bool aForAllTypes, bool *_retval); \
  nsresult SetDefaultBrowser(bool aClaimAllTypes, bool aForAllUsers); \
  nsresult SetDesktopBackground(mozilla::dom::Element *aElement, int32_t aPosition, const nsACString& aImageName); \
  nsresult GetDesktopBackgroundColor(uint32_t *aDesktopBackgroundColor); \
  nsresult SetDesktopBackgroundColor(uint32_t aDesktopBackgroundColor); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISHELLSERVICE(_to) \
  NS_IMETHOD IsDefaultBrowser(bool aForAllTypes, bool *_retval) override { return _to IsDefaultBrowser(aForAllTypes, _retval); } \
  NS_IMETHOD SetDefaultBrowser(bool aClaimAllTypes, bool aForAllUsers) override { return _to SetDefaultBrowser(aClaimAllTypes, aForAllUsers); } \
  NS_IMETHOD SetDesktopBackground(mozilla::dom::Element *aElement, int32_t aPosition, const nsACString& aImageName) override { return _to SetDesktopBackground(aElement, aPosition, aImageName); } \
  NS_IMETHOD GetDesktopBackgroundColor(uint32_t *aDesktopBackgroundColor) override { return _to GetDesktopBackgroundColor(aDesktopBackgroundColor); } \
  NS_IMETHOD SetDesktopBackgroundColor(uint32_t aDesktopBackgroundColor) override { return _to SetDesktopBackgroundColor(aDesktopBackgroundColor); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISHELLSERVICE(_to) \
  NS_IMETHOD IsDefaultBrowser(bool aForAllTypes, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsDefaultBrowser(aForAllTypes, _retval); } \
  NS_IMETHOD SetDefaultBrowser(bool aClaimAllTypes, bool aForAllUsers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultBrowser(aClaimAllTypes, aForAllUsers); } \
  NS_IMETHOD SetDesktopBackground(mozilla::dom::Element *aElement, int32_t aPosition, const nsACString& aImageName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDesktopBackground(aElement, aPosition, aImageName); } \
  NS_IMETHOD GetDesktopBackgroundColor(uint32_t *aDesktopBackgroundColor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDesktopBackgroundColor(aDesktopBackgroundColor); } \
  NS_IMETHOD SetDesktopBackgroundColor(uint32_t aDesktopBackgroundColor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDesktopBackgroundColor(aDesktopBackgroundColor); } 


#endif /* __gen_nsIShellService_h__ */
