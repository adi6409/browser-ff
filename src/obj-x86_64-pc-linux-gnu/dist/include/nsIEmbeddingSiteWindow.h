/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIEmbeddingSiteWindow.idl
 */

#ifndef __gen_nsIEmbeddingSiteWindow_h__
#define __gen_nsIEmbeddingSiteWindow_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIEmbeddingSiteWindow */
#define NS_IEMBEDDINGSITEWINDOW_IID_STR "0b976267-4aaa-4f36-a2d4-27b5ca8d73bb"

#define NS_IEMBEDDINGSITEWINDOW_IID \
  {0x0b976267, 0x4aaa, 0x4f36, \
    { 0xa2, 0xd4, 0x27, 0xb5, 0xca, 0x8d, 0x73, 0xbb }}

class NS_NO_VTABLE nsIEmbeddingSiteWindow : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEMBEDDINGSITEWINDOW_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEmbeddingSiteWindow;

  enum {
    DIM_FLAGS_POSITION = 1U,
    DIM_FLAGS_SIZE_INNER = 2U,
    DIM_FLAGS_SIZE_OUTER = 4U,
    DIM_FLAGS_IGNORE_X = 8U,
    DIM_FLAGS_IGNORE_Y = 16U,
    DIM_FLAGS_IGNORE_CX = 32U,
    DIM_FLAGS_IGNORE_CY = 64U
  };

  /* void setDimensions (in unsigned long flags, in long x, in long y, in long cx, in long cy); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDimensions(uint32_t flags, int32_t x, int32_t y, int32_t cx, int32_t cy) = 0;

  /* void getDimensions (in unsigned long flags, out long x, out long y, out long cx, out long cy); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDimensions(uint32_t flags, int32_t *x, int32_t *y, int32_t *cx, int32_t *cy) = 0;

  /* void setFocus (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocus(void) = 0;

  /* attribute boolean visibility; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVisibility(bool *aVisibility) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetVisibility(bool aVisibility) = 0;

  /* attribute AString title; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTitle(nsAString& aTitle) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTitle(const nsAString& aTitle) = 0;

  /* [noscript] readonly attribute voidPtr siteWindow; */
  NS_IMETHOD GetSiteWindow(void * * aSiteWindow) = 0;

  /* void blur (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Blur(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEmbeddingSiteWindow, NS_IEMBEDDINGSITEWINDOW_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEMBEDDINGSITEWINDOW \
  NS_IMETHOD SetDimensions(uint32_t flags, int32_t x, int32_t y, int32_t cx, int32_t cy) override; \
  NS_IMETHOD GetDimensions(uint32_t flags, int32_t *x, int32_t *y, int32_t *cx, int32_t *cy) override; \
  NS_IMETHOD SetFocus(void) override; \
  NS_IMETHOD GetVisibility(bool *aVisibility) override; \
  NS_IMETHOD SetVisibility(bool aVisibility) override; \
  NS_IMETHOD GetTitle(nsAString& aTitle) override; \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override; \
  NS_IMETHOD GetSiteWindow(void * * aSiteWindow) override; \
  NS_IMETHOD Blur(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEMBEDDINGSITEWINDOW \
  nsresult SetDimensions(uint32_t flags, int32_t x, int32_t y, int32_t cx, int32_t cy); \
  nsresult GetDimensions(uint32_t flags, int32_t *x, int32_t *y, int32_t *cx, int32_t *cy); \
  nsresult SetFocus(void); \
  nsresult GetVisibility(bool *aVisibility); \
  nsresult SetVisibility(bool aVisibility); \
  nsresult GetTitle(nsAString& aTitle); \
  nsresult SetTitle(const nsAString& aTitle); \
  nsresult GetSiteWindow(void * * aSiteWindow); \
  nsresult Blur(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEMBEDDINGSITEWINDOW(_to) \
  NS_IMETHOD SetDimensions(uint32_t flags, int32_t x, int32_t y, int32_t cx, int32_t cy) override { return _to SetDimensions(flags, x, y, cx, cy); } \
  NS_IMETHOD GetDimensions(uint32_t flags, int32_t *x, int32_t *y, int32_t *cx, int32_t *cy) override { return _to GetDimensions(flags, x, y, cx, cy); } \
  NS_IMETHOD SetFocus(void) override { return _to SetFocus(); } \
  NS_IMETHOD GetVisibility(bool *aVisibility) override { return _to GetVisibility(aVisibility); } \
  NS_IMETHOD SetVisibility(bool aVisibility) override { return _to SetVisibility(aVisibility); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return _to GetTitle(aTitle); } \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override { return _to SetTitle(aTitle); } \
  NS_IMETHOD GetSiteWindow(void * * aSiteWindow) override { return _to GetSiteWindow(aSiteWindow); } \
  NS_IMETHOD Blur(void) override { return _to Blur(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEMBEDDINGSITEWINDOW(_to) \
  NS_IMETHOD SetDimensions(uint32_t flags, int32_t x, int32_t y, int32_t cx, int32_t cy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDimensions(flags, x, y, cx, cy); } \
  NS_IMETHOD GetDimensions(uint32_t flags, int32_t *x, int32_t *y, int32_t *cx, int32_t *cy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDimensions(flags, x, y, cx, cy); } \
  NS_IMETHOD SetFocus(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFocus(); } \
  NS_IMETHOD GetVisibility(bool *aVisibility) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisibility(aVisibility); } \
  NS_IMETHOD SetVisibility(bool aVisibility) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetVisibility(aVisibility); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTitle(aTitle); } \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTitle(aTitle); } \
  NS_IMETHOD GetSiteWindow(void * * aSiteWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSiteWindow(aSiteWindow); } \
  NS_IMETHOD Blur(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Blur(); } 


#endif /* __gen_nsIEmbeddingSiteWindow_h__ */
