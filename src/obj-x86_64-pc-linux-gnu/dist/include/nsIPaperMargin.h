/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPaperMargin.idl
 */

#ifndef __gen_nsIPaperMargin_h__
#define __gen_nsIPaperMargin_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPaperMargin */
#define NS_IPAPERMARGIN_IID_STR "0858d1a7-b646-4b15-a1e8-7eb5ab572d0a"

#define NS_IPAPERMARGIN_IID \
  {0x0858d1a7, 0xb646, 0x4b15, \
    { 0xa1, 0xe8, 0x7e, 0xb5, 0xab, 0x57, 0x2d, 0x0a }}

class NS_NO_VTABLE nsIPaperMargin : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAPERMARGIN_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaperMargin;

  /* [infallible] readonly attribute double top; */
  NS_IMETHOD GetTop(double *aTop) = 0;
  inline double  GetTop()
  {
    double result;
    mozilla::DebugOnly<nsresult> rv = GetTop(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute double right; */
  NS_IMETHOD GetRight(double *aRight) = 0;
  inline double  GetRight()
  {
    double result;
    mozilla::DebugOnly<nsresult> rv = GetRight(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute double bottom; */
  NS_IMETHOD GetBottom(double *aBottom) = 0;
  inline double  GetBottom()
  {
    double result;
    mozilla::DebugOnly<nsresult> rv = GetBottom(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute double left; */
  NS_IMETHOD GetLeft(double *aLeft) = 0;
  inline double  GetLeft()
  {
    double result;
    mozilla::DebugOnly<nsresult> rv = GetLeft(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaperMargin, NS_IPAPERMARGIN_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAPERMARGIN \
  using nsIPaperMargin::GetTop; \
  NS_IMETHOD GetTop(double *aTop) override; \
  using nsIPaperMargin::GetRight; \
  NS_IMETHOD GetRight(double *aRight) override; \
  using nsIPaperMargin::GetBottom; \
  NS_IMETHOD GetBottom(double *aBottom) override; \
  using nsIPaperMargin::GetLeft; \
  NS_IMETHOD GetLeft(double *aLeft) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAPERMARGIN \
  using nsIPaperMargin::GetTop; \
  nsresult GetTop(double *aTop); \
  using nsIPaperMargin::GetRight; \
  nsresult GetRight(double *aRight); \
  using nsIPaperMargin::GetBottom; \
  nsresult GetBottom(double *aBottom); \
  using nsIPaperMargin::GetLeft; \
  nsresult GetLeft(double *aLeft); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAPERMARGIN(_to) \
  using nsIPaperMargin::GetTop; \
  NS_IMETHOD GetTop(double *aTop) override { return _to GetTop(aTop); } \
  using nsIPaperMargin::GetRight; \
  NS_IMETHOD GetRight(double *aRight) override { return _to GetRight(aRight); } \
  using nsIPaperMargin::GetBottom; \
  NS_IMETHOD GetBottom(double *aBottom) override { return _to GetBottom(aBottom); } \
  using nsIPaperMargin::GetLeft; \
  NS_IMETHOD GetLeft(double *aLeft) override { return _to GetLeft(aLeft); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAPERMARGIN(_to) \
  NS_IMETHOD GetTop(double *aTop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTop(aTop); } \
  NS_IMETHOD GetRight(double *aRight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRight(aRight); } \
  NS_IMETHOD GetBottom(double *aBottom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBottom(aBottom); } \
  NS_IMETHOD GetLeft(double *aLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLeft(aLeft); } 


#endif /* __gen_nsIPaperMargin_h__ */
