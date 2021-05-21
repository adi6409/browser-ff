/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleImage.idl
 */

#ifndef __gen_nsIAccessibleImage_h__
#define __gen_nsIAccessibleImage_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleImage */
#define NS_IACCESSIBLEIMAGE_IID_STR "09086623-0f09-4310-ac56-c2cda7c29648"

#define NS_IACCESSIBLEIMAGE_IID \
  {0x09086623, 0x0f09, 0x4310, \
    { 0xac, 0x56, 0xc2, 0xcd, 0xa7, 0xc2, 0x96, 0x48 }}

class NS_NO_VTABLE nsIAccessibleImage : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEIMAGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleImage;

  /* void getImagePosition (in unsigned long coordType, out long x, out long y); */
  NS_IMETHOD GetImagePosition(uint32_t coordType, int32_t *x, int32_t *y) = 0;

  /* void getImageSize (out long width, out long height); */
  NS_IMETHOD GetImageSize(int32_t *width, int32_t *height) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleImage, NS_IACCESSIBLEIMAGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEIMAGE \
  NS_IMETHOD GetImagePosition(uint32_t coordType, int32_t *x, int32_t *y) override; \
  NS_IMETHOD GetImageSize(int32_t *width, int32_t *height) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEIMAGE \
  nsresult GetImagePosition(uint32_t coordType, int32_t *x, int32_t *y); \
  nsresult GetImageSize(int32_t *width, int32_t *height); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEIMAGE(_to) \
  NS_IMETHOD GetImagePosition(uint32_t coordType, int32_t *x, int32_t *y) override { return _to GetImagePosition(coordType, x, y); } \
  NS_IMETHOD GetImageSize(int32_t *width, int32_t *height) override { return _to GetImageSize(width, height); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEIMAGE(_to) \
  NS_IMETHOD GetImagePosition(uint32_t coordType, int32_t *x, int32_t *y) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImagePosition(coordType, x, y); } \
  NS_IMETHOD GetImageSize(int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImageSize(width, height); } 


#endif /* __gen_nsIAccessibleImage_h__ */
