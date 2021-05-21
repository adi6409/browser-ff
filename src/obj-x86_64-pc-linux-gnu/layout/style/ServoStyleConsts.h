/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// See the comment in ServoBindings.h about the same.
#pragma GCC diagnostic push
#ifdef __clang__
#  pragma GCC diagnostic ignored "-Wreturn-type-c-linkage"
#endif
// Work-around silly windows.h define.
#pragma push_macro("STRICT")
#undef STRICT


#ifndef mozilla_ServoStyleConsts_h
#define mozilla_ServoStyleConsts_h

/* Generated with cbindgen:0.16.0 */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `rustup run nightly cbindgen toolkit/library/rust/ --lockfile Cargo.lock --crate style -o layout/style/ServoStyleConsts.h`
 */


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include "mozilla/ServoStyleConstsForwards.h"
#include "mozilla/ServoStyleSet.h"

namespace mozilla {

/// The minimum stack size for a thread in the styling pool, in kilobytes.
static const uintptr_t StyleSTYLE_THREAD_STACK_SIZE_KB = 256;

/// The stack margin. If we get this deep in the stack, we will skip recursive
/// optimizations to ensure that there is sufficient room for non-recursive work.
///
/// We allocate large safety margins because certain OS calls can use very large
/// amounts of stack space [1]. Reserving a larger-than-necessary stack costs us
/// address space, but if we keep our safety margin big, we will generally avoid
/// committing those extra pages, and only use them in edge cases that would
/// otherwise cause crashes.
///
/// When measured with 128KB stacks and 40KB margin, we could support 53
/// levels of recursion before the limiter kicks in, on x86_64-Linux [2]. When
/// we doubled the stack size, we added it all to the safety margin, so we should
/// be able to get the same amount of recursion.
///
/// [1] https://bugzilla.mozilla.org/show_bug.cgi?id=1395708#c15
/// [2] See Gecko bug 1376883 for more discussion on the measurements.
///
static const uintptr_t StyleSTACK_SAFETY_MARGIN_KB = 168;

/// The maximum number of child nodes that we will process as a single unit.
///
/// Larger values will increase style sharing cache hits and general DOM
/// locality at the expense of decreased opportunities for parallelism.  There
/// are some measurements in
/// https://bugzilla.mozilla.org/show_bug.cgi?id=1385982#c11 and comments 12
/// and 13 that investigate some slightly different values for the work unit
/// size.  If the size is significantly increased, make sure to adjust the
/// condition for kicking off a new work unit in top_down_dom, because
/// otherwise we're likely to end up doing too much work serially.  For
/// example, the condition there could become some fraction of WORK_UNIT_MAX
/// instead of WORK_UNIT_MAX.
static const uintptr_t StyleWORK_UNIT_MAX = 16;

/// The amount of nodes that the style sharing candidate cache should hold at
/// most.
///
/// The cache size was chosen by measuring style sharing and resulting
/// performance on a few pages; sizes up to about 32 were giving good sharing
/// improvements (e.g. 3x fewer styles having to be resolved than at size 8) and
/// slight performance improvements.  Sizes larger than 32 haven't really been
/// tested.
static const uintptr_t StyleSHARING_CACHE_SIZE = 32;

static const uint8_t StyleLengthPercentageUnion_TAG_CALC = 0;

static const uint8_t StyleLengthPercentageUnion_TAG_LENGTH = 1;

static const uint8_t StyleLengthPercentageUnion_TAG_PERCENTAGE = 2;

static const uint8_t StyleLengthPercentageUnion_TAG_MASK = 3;

/// These are the limits that we choose to clamp grid line numbers to.
/// http://drafts.csswg.org/css-grid/#overlarge-grids
/// line_num is clamped to this range at parse time.
static const int32_t StyleMIN_GRID_LINE = -10000;

/// See above.
static const int32_t StyleMAX_GRID_LINE = 10000;

/// The minimum font-weight value per:
///
/// https://drafts.csswg.org/css-fonts-4/#font-weight-numeric-values
static const float StyleMIN_FONT_WEIGHT = 1.;

/// The maximum font-weight value per:
///
/// https://drafts.csswg.org/css-fonts-4/#font-weight-numeric-values
static const float StyleMAX_FONT_WEIGHT = 1000.;

/// The default angle for `font-style: oblique`.
///
/// NOTE(emilio): As of right now this diverges from the spec, which specifies
/// 20, because it's not updated yet to account for the resolution in:
///
///   https://github.com/w3c/csswg-drafts/issues/2295
static const float StyleDEFAULT_FONT_STYLE_OBLIQUE_ANGLE_DEGREES = 14.;

/// From https://drafts.csswg.org/css-fonts-4/#valdef-font-style-oblique-angle:
///
///     Values less than -90deg or values greater than 90deg are
///     invalid and are treated as parse errors.
///
/// The maximum angle value that `font-style: oblique` should compute to.
static const float StyleFONT_STYLE_OBLIQUE_MAX_ANGLE_DEGREES = 90.;

/// The minimum angle value that `font-style: oblique` should compute to.
static const float StyleFONT_STYLE_OBLIQUE_MIN_ANGLE_DEGREES = -90.;

/// The default font size.
static const float StyleFONT_MEDIUM_PX = 16.0;

/// Number of non-normal components
static const uint8_t StylePAINT_ORDER_COUNT = 3;

/// Number of bits for each component
static const uint8_t StylePAINT_ORDER_SHIFT = 2;

/// Mask with above bits set
static const uint8_t StylePAINT_ORDER_MASK = 3;

#if defined(CBINDGEN_IS_SERVO)
/// The number of eager pseudo-elements. Keep this in sync with cascade_type.
static const uintptr_t StyleEAGER_PSEUDO_COUNT = 3;
#endif

/// Whether @import rules are allowed.
enum class StyleAllowImportRules : uint8_t {
  /// @import rules will be parsed.
  Yes,
  /// @import rules will not be parsed.
  No,
};

/// Whether to allow negative lengths or not.
enum class StyleAllowedNumericType : uint8_t {
  /// Allow all kind of numeric values.
  All,
  /// Allow only non-negative numeric values.
  NonNegative,
  /// Allow only numeric values greater or equal to 1.0.
  AtLeastOne,
};

/// The value for the `appearance` property.
///
/// https://developer.mozilla.org/en-US/docs/Web/CSS/-moz-appearance
enum class StyleAppearance : uint8_t {
  /// No appearance at all.
  None,
  /// Default appearance for the element.
  ///
  /// This value doesn't make sense for -moz-default-appearance, but we don't bother to guard
  /// against parsing it.
  Auto,
  /// A searchfield.
  Searchfield,
  /// A multi-line text field, e.g. HTML <textarea>.
  Textarea,
  /// A checkbox element.
  Checkbox,
  /// A radio element within a radio group.
  Radio,
  /// A dropdown list.
  Menulist,
  /// List boxes.
  Listbox,
  /// A horizontal meter bar.
  Meter,
  /// A horizontal progress bar.
  ProgressBar,
  /// A typical dialog button.
  Button,
  /// A single-line text field, e.g. HTML <input type=text>.
  Textfield,
  /// The dropdown button(s) that open up a dropdown list.
  MenulistButton,
  /// Various arrows that go in buttons
  ButtonArrowDown,
  ButtonArrowNext,
  ButtonArrowPrevious,
  ButtonArrowUp,
  /// The focus outline box inside of a button.
  ButtonFocus,
  /// A dual toolbar button (e.g., a Back button with a dropdown)
  Dualbutton,
  /// A groupbox.
  Groupbox,
  /// Menu Bar background
  Menubar,
  /// <menu> and <menuitem> appearances
  Menuitem,
  Checkmenuitem,
  Radiomenuitem,
  /// For text on non-iconic menuitems only
  Menuitemtext,
  /// The text part of a dropdown list, to left of button.
  MenulistText,
  /// Menu Popup background.
  Menupopup,
  /// menu checkbox/radio appearances
  Menucheckbox,
  Menuradio,
  Menuseparator,
  Menuarrow,
  /// An image in the menu gutter, like in bookmarks or history.
  Menuimage,
  /// The meter bar's meter indicator.
  Meterchunk,
  /// The "arrowed" part of the dropdown button that open up a dropdown list.
  MozMenulistArrowButton,
  /// For HTML's <input type=number>
  NumberInput,
  /// The progress bar's progress indicator
  Progresschunk,
  /// A generic container that always repaints on state changes. This is a
  /// hack to make XUL checkboxes and radio buttons work.
  CheckboxContainer,
  RadioContainer,
  /// The label part of a checkbox or radio button, used for painting a focus
  /// outline.
  CheckboxLabel,
  RadioLabel,
  /// nsRangeFrame and its subparts
  Range,
  RangeThumb,
  /// The resizer background area in a status bar for the resizer widget in
  /// the corner of a window.
  Resizerpanel,
  /// The resizer itself.
  Resizer,
  /// The scrollbar slider
  ScrollbarHorizontal,
  ScrollbarVertical,
  /// A scrollbar button (up/down/left/right).
  /// Keep these in order (some code casts these values to `int` in order to
  /// compare them against each other).
  ScrollbarbuttonUp,
  ScrollbarbuttonDown,
  ScrollbarbuttonLeft,
  ScrollbarbuttonRight,
  /// The scrollbar thumb.
  ScrollbarthumbHorizontal,
  ScrollbarthumbVertical,
  /// The scrollbar track.
  ScrollbartrackHorizontal,
  ScrollbartrackVertical,
  /// The scroll corner
  Scrollcorner,
  /// A separator.  Can be horizontal or vertical.
  Separator,
  /// A spin control (up/down control for time/date pickers).
  Spinner,
  /// The up button of a spin control.
  SpinnerUpbutton,
  /// The down button of a spin control.
  SpinnerDownbutton,
  /// The textfield of a spin control
  SpinnerTextfield,
  /// A splitter.  Can be horizontal or vertical.
  Splitter,
  /// A status bar in a main application window.
  Statusbar,
  /// A single pane of a status bar.
  Statusbarpanel,
  /// A single tab in a tab widget.
  Tab,
  /// A single pane (inside the tabpanels container).
  Tabpanel,
  /// The tab panels container.
  Tabpanels,
  /// The tabs scroll arrows (left/right).
  TabScrollArrowBack,
  TabScrollArrowForward,
  /// A toolbar in an application window.
  Toolbar,
  /// A single toolbar button (with no associated dropdown).
  Toolbarbutton,
  /// The dropdown portion of a toolbar button
  ToolbarbuttonDropdown,
  /// The gripper for a toolbar.
  Toolbargripper,
  /// The toolbox that contains the toolbars.
  Toolbox,
  /// A tooltip.
  Tooltip,
  /// A listbox or tree widget header
  Treeheader,
  /// An individual header cell
  Treeheadercell,
  /// The sort arrow for a header.
  Treeheadersortarrow,
  /// A tree item.
  Treeitem,
  /// A tree widget branch line
  Treeline,
  /// A tree widget twisty.
  Treetwisty,
  /// Open tree widget twisty.
  Treetwistyopen,
  /// A tree widget.
  Treeview,
  /// Window and dialog backgrounds.
  Window,
  Dialog,
  /// Vista Rebars.
  MozWinCommunicationsToolbox,
  MozWinMediaToolbox,
  MozWinBrowsertabbarToolbox,
  /// Vista glass.
  MozWinGlass,
  MozWinBorderlessGlass,
  /// -moz-apperance style used in setting proper glass margins.
  MozWinExcludeGlass,
  /// Mac help button.
  MozMacHelpButton,
  /// Windows themed window frame elements.
  MozWindowButtonBox,
  MozWindowButtonBoxMaximized,
  MozWindowButtonClose,
  MozWindowButtonMaximize,
  MozWindowButtonMinimize,
  MozWindowButtonRestore,
  MozWindowFrameBottom,
  MozWindowFrameLeft,
  MozWindowFrameRight,
  MozWindowTitlebar,
  MozWindowTitlebarMaximized,
  MozGtkInfoBar,
  MozMacActiveSourceListSelection,
  MozMacDisclosureButtonClosed,
  MozMacDisclosureButtonOpen,
  MozMacSourceList,
  MozMacSourceListSelection,
  MozMacVibrancyDark,
  MozMacVibrancyLight,
  MozMacVibrantTitlebarDark,
  MozMacVibrantTitlebarLight,
  /// A themed focus outline (for outline:auto).
  ///
  /// This isn't exposed to CSS at all, just here for convenience.
  FocusOutline,
  /// A dummy variant that should be last to let the GTK widget do hackery.
  Count,
};

/// A specified value for a single side of a `border-style` property.
///
/// The order here corresponds to the integer values from the border conflict
/// resolution rules in CSS 2.1 § 17.6.2.1. Higher values override lower values.
enum class StyleBorderStyle : uint8_t {
  Hidden,
  None,
  Inset,
  Groove,
  Outset,
  Ridge,
  Dotted,
  Dashed,
  Solid,
  Double,
};

/// A kind of break between two boxes.
///
/// https://drafts.csswg.org/css-break/#break-between
enum class StyleBreakBetween : uint8_t {
  Always,
  Auto,
  Page,
  Avoid,
  Left,
  Right,
};

/// A kind of break within a box.
///
/// https://drafts.csswg.org/css-break/#break-within
enum class StyleBreakWithin : uint8_t {
  Auto,
  Avoid,
};

enum class StyleCaptionSide : uint8_t {
  Top,
  Bottom,
#if defined(CBINDGEN_IS_GECKO)
  Right,
#endif
#if defined(CBINDGEN_IS_GECKO)
  Left,
#endif
#if defined(CBINDGEN_IS_GECKO)
  TopOutside,
#endif
#if defined(CBINDGEN_IS_GECKO)
  BottomOutside,
#endif
};

#if defined(CBINDGEN_IS_GECKO)
/// Represents the parts of prefers-contrast that explicitly deal with
/// contrast. Used in combination with information about rather or not
/// forced colors are active this allows for evaluation of the
/// prefers-contrast media query.
enum class StyleContrastPref : uint8_t {
#if defined(CBINDGEN_IS_GECKO)
  /// More contrast is prefered. Corresponds to an accessibility theme
  /// being enabled or firefox forcing high contrast colors.
  More,
#endif
#if defined(CBINDGEN_IS_GECKO)
  /// Low contrast is prefered.
  Less,
#endif
#if defined(CBINDGEN_IS_GECKO)
  /// The default value if neither high or low contrast is enabled.
  NoPreference,
#endif
};
#endif

/// The CORS mode used for a CSS load.
enum class StyleCorsMode : uint8_t {
  /// No CORS mode, so cross-origin loads can be done.
  None,
  /// Anonymous CORS request.
  Anonymous,
};

enum class StyleCounterSystem : uint8_t {
  Cyclic = 0,
  Numeric,
  Alphabetic,
  Symbolic,
  Additive,
  Fixed,
  Extends,
};

/// The keywords allowed in the Cursor property.
///
/// https://drafts.csswg.org/css-ui-4/#propdef-cursor
enum class StyleCursorKind : uint8_t {
  None,
  Default,
  Pointer,
  ContextMenu,
  Help,
  Progress,
  Wait,
  Cell,
  Crosshair,
  Text,
  VerticalText,
  Alias,
  Copy,
  Move,
  NoDrop,
  NotAllowed,
  Grab,
  Grabbing,
  EResize,
  NResize,
  NeResize,
  NwResize,
  SResize,
  SeResize,
  SwResize,
  WResize,
  EwResize,
  NsResize,
  NeswResize,
  NwseResize,
  ColResize,
  RowResize,
  AllScroll,
  ZoomIn,
  ZoomOut,
  Auto,
};

enum class StyleDisplayInside : uint8_t {
  None = 0,
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  Contents,
#endif
  Flow,
  FlowRoot,
  Flex,
#if defined(CBINDGEN_IS_GECKO)
  Grid,
#endif
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  Table,
#endif
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  TableRowGroup,
#endif
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  TableColumn,
#endif
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  TableColumnGroup,
#endif
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  TableHeaderGroup,
#endif
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  TableFooterGroup,
#endif
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  TableRow,
#endif
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  TableCell,
#endif
#if defined(CBINDGEN_IS_GECKO)
  Ruby,
#endif
#if defined(CBINDGEN_IS_GECKO)
  RubyBase,
#endif
#if defined(CBINDGEN_IS_GECKO)
  RubyBaseContainer,
#endif
#if defined(CBINDGEN_IS_GECKO)
  RubyText,
#endif
#if defined(CBINDGEN_IS_GECKO)
  RubyTextContainer,
#endif
#if defined(CBINDGEN_IS_GECKO)
  WebkitBox,
#endif
#if defined(CBINDGEN_IS_GECKO)
  MozBox,
#endif
#if defined(CBINDGEN_IS_GECKO)
  MozStack,
#endif
#if defined(CBINDGEN_IS_GECKO)
  MozDeck,
#endif
#if defined(CBINDGEN_IS_GECKO)
  MozPopup,
#endif
};

#if defined(CBINDGEN_IS_GECKO)
/// Values for the display-mode media feature.
enum class StyleDisplayMode : uint8_t {
#if defined(CBINDGEN_IS_GECKO)
  Browser = 0,
#endif
#if defined(CBINDGEN_IS_GECKO)
  MinimalUi,
#endif
#if defined(CBINDGEN_IS_GECKO)
  Standalone,
#endif
#if defined(CBINDGEN_IS_GECKO)
  Fullscreen,
#endif
};
#endif

/// Defines an element’s display type, which consists of
/// the two basic qualities of how an element generates boxes
/// <https://drafts.csswg.org/css-display/#propdef-display>
enum class StyleDisplayOutside : uint8_t {
  None = 0,
  Inline,
  Block,
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  TableCaption,
#endif
#if (defined(CBINDGEN_IS_SERVO) || defined(CBINDGEN_IS_GECKO))
  InternalTable,
#endif
#if defined(CBINDGEN_IS_GECKO)
  InternalRuby,
#endif
#if defined(CBINDGEN_IS_GECKO)
  XUL,
#endif
};

/// A type for possible values for min- and max- flavors of width, height,
/// block-size, and inline-size.
enum class StyleExtremumLength : uint8_t {
  MaxContent,
  MinContent,
  MozFitContent,
  MozAvailable,
};

enum class StyleFillRule : uint8_t {
  Nonzero,
  Evenodd,
};

/// A computed value for the `float` property.
enum class StyleFloat : uint8_t {
  Left,
  Right,
  None,
};

/// A font-display value for a @font-face rule.
/// The font-display descriptor determines how a font face is displayed based
/// on whether and when it is downloaded and ready to use.
enum class StyleFontDisplay : uint8_t {
  Auto,
  Block,
  Swap,
  Fallback,
  Optional,
};

/// Font family names must either be given quoted as strings,
/// or unquoted as a sequence of one or more identifiers.
enum class StyleFontFamilyNameSyntax : uint8_t {
  /// The family name was specified in a quoted form, e.g. "Font Name"
  /// or 'Font Name'.
  Quoted,
  /// The family name was specified in an unquoted form as a sequence of
  /// identifiers.
  Identifiers,
};

/// CSS font keywords
enum class StyleFontSizeKeyword : uint8_t {
  XXSmall,
  XSmall,
  Small,
  Medium,
  Large,
  XLarge,
  XXLarge,
  XXXLarge,
  None,
};

/// A generic font-family name.
///
/// The order here is important, if you change it make sure that
/// `gfxPlatformFontList.h`s ranged array and `gfxFontFamilyList`'s
/// sSingleGenerics are updated as well.
enum class StyleGenericFontFamily : uint8_t {
  /// No generic family specified, only for internal usage.
  ///
  /// NOTE(emilio): Gecko code relies on this variant being zero.
  None = 0,
  Serif,
  SansSerif,
  Monospace,
  Cursive,
  Fantasy,
#if defined(CBINDGEN_IS_GECKO)
  /// An internal value for emoji font selection.
  MozEmoji,
#endif
};

/// Whether we used the modern notation or the compatibility `-webkit`, `-moz` prefixes.
enum class StyleGradientCompatMode : uint8_t {
  /// Modern syntax.
  Modern,
  /// `-webkit` prefix.
  WebKit,
  /// `-moz` prefix
  Moz,
};

/// A keyword for the X direction.
enum class StyleHorizontalPositionKeyword : uint8_t {
  Left,
  Right,
};

/// The path command absolute type.
enum class StyleIsAbsolute : uint8_t {
  Yes,
  No,
};

enum class StyleIsOrdinalInRange : uint8_t {
  Auto,
  InRange,
  NotInRange,
  NoOrdinalSpecified,
};

/// Values for the `line-break` property.
enum class StyleLineBreak : uint8_t {
  Auto,
  Loose,
  Normal,
  Strict,
  Anywhere,
};

/// Whether we're a `min` or `max` function.
enum class StyleMinMaxOp : uint8_t {
  /// `min()`
  Min,
  /// `max()`
  Max,
};

/// Specified and computed `-moz-list-reversed` property (for UA sheets only).
enum class StyleMozListReversed : uint8_t {
  /// the initial value
  False,
  /// exclusively used for <ol reversed> in our html.css UA sheet
  True,
};

/// Each style rule has an origin, which determines where it enters the cascade.
///
/// <https://drafts.csswg.org/css-cascade/#cascading-origins>
enum class StyleOrigin : uint8_t {
  /// <https://drafts.csswg.org/css-cascade/#cascade-origin-user-agent>
  UserAgent = 1,
  /// <https://drafts.csswg.org/css-cascade/#cascade-origin-user>
  User = 2,
  /// <https://drafts.csswg.org/css-cascade/#cascade-origin-author>
  Author = 4,
};

/// The value for the `overflow-x` / `overflow-y` properties.
enum class StyleOverflow : uint8_t {
  Visible,
  Hidden,
  Scroll,
  Auto,
#if defined(CBINDGEN_IS_GECKO)
  Clip,
#endif
};

enum class StyleOverflowAnchor : uint8_t {
  Auto,
  None,
};

enum class StyleOverflowClipBox : uint8_t {
  PaddingBox,
  ContentBox,
};

/// Values for the `overflow-wrap` property.
enum class StyleOverflowWrap : uint8_t {
  Normal,
  BreakWord,
  Anywhere,
};

enum class StyleOverscrollBehavior : uint8_t {
  Auto,
  Contain,
  None,
};

/// The specified value for a single CSS paint-order property.
enum class StylePaintOrder : uint8_t {
  /// `normal` variant
  Normal = 0,
  /// `fill` variant
  Fill = 1,
  /// `stroke` variant
  Stroke = 2,
  /// `markers` variant
  Markers = 3,
};

#if defined(CBINDGEN_IS_GECKO)
/// Values for the prefers-color-scheme media feature.
enum class StylePrefersColorScheme : uint8_t {
#if defined(CBINDGEN_IS_GECKO)
  Light,
#endif
#if defined(CBINDGEN_IS_GECKO)
  Dark,
#endif
#if defined(CBINDGEN_IS_GECKO)
  NoPreference,
#endif
};
#endif

/// The <size> in ray() function.
///
/// https://drafts.fxtf.org/motion-1/#valdef-offsetpath-size
enum class StyleRaySize : uint8_t {
  ClosestSide,
  ClosestCorner,
  FarthestSide,
  FarthestCorner,
  Sides,
};

/// A computed value for the `resize` property.
enum class StyleResize : uint8_t {
  None,
  Both,
  Horizontal,
  Vertical,
};

/// The kind of change that happened for a given rule.
enum class StyleRuleChangeKind : uint32_t {
  /// The rule was inserted.
  Insertion,
  /// The rule was removed.
  Removal,
  /// Some change in the rule which we don't know about, and could have made
  /// the rule change in any way.
  Generic,
  /// A change in the declarations of a style rule.
  StyleRuleDeclarations,
};

/// The kind of sanitization to use when parsing a stylesheet.
enum class StyleSanitizationKind : uint8_t {
  /// Perform no sanitization.
  None,
  /// Allow only @font-face, style rules, and @namespace.
  Standard,
  /// Allow everything but conditional rules.
  NoConditionalRules,
};

/// Specified value of scroll-snap-align keyword value.
enum class StyleScrollSnapAlignKeyword : uint8_t {
  None,
  Start,
  End,
  Center,
};

/// https://drafts.csswg.org/css-scroll-snap-1/#snap-axis
enum class StyleScrollSnapAxis : uint8_t {
  X,
  Y,
  Block,
  Inline,
  Both,
};

/// https://drafts.csswg.org/css-scroll-snap-1/#snap-strictness
enum class StyleScrollSnapStrictness : uint8_t {
  None,
  Mandatory,
  Proximity,
};

/// https://drafts.csswg.org/css-shapes-1/#typedef-shape-box
enum class StyleShapeBox : uint8_t {
  MarginBox,
  BorderBox,
  PaddingBox,
  ContentBox,
};

/// <https://drafts.csswg.org/css-images/#typedef-extent-keyword>
enum class StyleShapeExtent : uint8_t {
  ClosestSide,
  FarthestSide,
  ClosestCorner,
  FarthestCorner,
  Contain,
  Cover,
};

enum class StyleStepPosition : uint8_t {
  JumpStart,
  JumpEnd,
  JumpNone,
  JumpBoth,
  Start,
  End,
};

/// https://drafts.csswg.org/css-counter-styles/#typedef-symbols-type
enum class StyleSymbolsType : uint8_t {
  Cyclic,
  Numeric,
  Alphabetic,
  Symbolic,
  Fixed,
};

#if defined(CBINDGEN_IS_GECKO)
/// System colors.
enum class StyleSystemColor : uint8_t {
  WindowBackground,
  WindowForeground,
  WidgetBackground,
  WidgetForeground,
  WidgetSelectBackground,
  WidgetSelectForeground,
  Widget3DHighlight,
  Widget3DShadow,
  TextBackground,
  TextForeground,
  TextSelectBackground,
  TextSelectForeground,
  TextSelectForegroundCustom,
  TextSelectBackgroundDisabled,
  TextSelectBackgroundAttention,
  TextHighlightBackground,
  TextHighlightForeground,
  IMERawInputBackground,
  IMERawInputForeground,
  IMERawInputUnderline,
  IMESelectedRawTextBackground,
  IMESelectedRawTextForeground,
  IMESelectedRawTextUnderline,
  IMEConvertedTextBackground,
  IMEConvertedTextForeground,
  IMEConvertedTextUnderline,
  IMESelectedConvertedTextBackground,
  IMESelectedConvertedTextForeground,
  IMESelectedConvertedTextUnderline,
  SpellCheckerUnderline,
  ThemedScrollbar,
  ThemedScrollbarInactive,
  ThemedScrollbarThumb,
  ThemedScrollbarThumbHover,
  ThemedScrollbarThumbActive,
  ThemedScrollbarThumbInactive,
  Activeborder,
  Activecaption,
  Appworkspace,
  Background,
  Buttonface,
  Buttonhighlight,
  Buttonshadow,
  Buttontext,
  Captiontext,
  Field,
  Fieldtext,
  Graytext,
  Highlight,
  Highlighttext,
  Inactiveborder,
  Inactivecaption,
  Inactivecaptiontext,
  Infobackground,
  Infotext,
  Menu,
  Menutext,
  Scrollbar,
  Threeddarkshadow,
  Threedface,
  Threedhighlight,
  Threedlightshadow,
  Threedshadow,
  Window,
  Windowframe,
  Windowtext,
  MozButtondefault,
  Canvastext,
  Canvas,
  MozDialog,
  MozDialogtext,
  /// Used to highlight valid regions to drop something onto.
  MozDragtargetzone,
  /// Used for selected but not focused cell backgrounds.
  MozCellhighlight,
  /// Used for selected but not focused cell text.
  MozCellhighlighttext,
  /// Used for selected but not focused html cell backgrounds.
  MozHtmlCellhighlight,
  /// Used for selected but not focused html cell text.
  MozHtmlCellhighlighttext,
  /// Used to button text background when hovered.
  MozButtonhoverface,
  /// Used to button text color when hovered.
  MozButtonhovertext,
  /// Used for menu item backgrounds when hovered.
  MozMenuhover,
  /// Used for menu item text when hovered.
  MozMenuhovertext,
  /// Used for menubar item text.
  MozMenubartext,
  /// Used for menubar item text when hovered.
  MozMenubarhovertext,
  /// On platforms where these colors are the same as -moz-field, use
  /// -moz-fieldtext as foreground color
  MozEventreerow,
  MozOddtreerow,
  /// Used for button text when pressed.
  MozGtkButtonactivetext,
  /// Used for button text when pressed.
  MozMacButtonactivetext,
  /// Background color of chrome toolbars in active windows.
  MozMacChromeActive,
  /// Background color of chrome toolbars in inactive windows.
  MozMacChromeInactive,
  /// Foreground color of default buttons.
  MozMacDefaultbuttontext,
  /// Ring color around text fields and lists.
  MozMacFocusring,
  /// Color used when mouse is over a menu item.
  MozMacMenuselect,
  /// Color used to do shadows on menu items.
  MozMacMenushadow,
  /// Color used to display text for disabled menu items.
  MozMacMenutextdisable,
  /// Color used to display text while mouse is over a menu item.
  MozMacMenutextselect,
  /// Text color of disabled text on toolbars.
  MozMacDisabledtoolbartext,
  /// Inactive light hightlight
  MozMacSecondaryhighlight,
  /// Font smoothing background colors needed by the Mac OS X theme, based on
  /// -moz-appearance names.
  MozMacVibrancyLight,
  MozMacVibrancyDark,
  MozMacVibrantTitlebarLight,
  MozMacVibrantTitlebarDark,
  MozMacMenupopup,
  MozMacMenuitem,
  MozMacActiveMenuitem,
  MozMacSourceList,
  MozMacSourceListSelection,
  MozMacActiveSourceListSelection,
  MozMacTooltip,
  /// Theme accent color.
  MozAccentColor,
  /// Foreground for the accent color.
  MozAccentColorForeground,
  /// Accent color for title bar.
  MozWinAccentcolor,
  /// Color from drawing text over the accent color.
  MozWinAccentcolortext,
  /// Media rebar text.
  MozWinMediatext,
  /// Communications rebar text.
  MozWinCommunicationstext,
  /// Hyperlink color extracted from the system, not affected by the
  /// browser.anchor_color user pref.
  ///
  /// There is no OS-specified safe background color for this text, but it is
  /// used regularly within Windows and the Gnome DE on Dialog and Window
  /// colors.
  MozNativehyperlinktext,
  Linktext,
  Activetext,
  Visitedtext,
  /// Combobox widgets
  MozComboboxtext,
  MozCombobox,
  MozGtkInfoBarText,
  /// Color of tree column headers
  MozColheadertext,
  MozColheaderhovertext,
  End,
};
#endif

/// Specified value of text-align keyword value.
enum class StyleTextAlignKeyword : uint8_t {
  Start,
  Left,
  Right,
  Center,
#if (defined(CBINDGEN_IS_GECKO) || defined(CBINDGEN_IS_SERVO))
  Justify,
#endif
#if defined(CBINDGEN_IS_GECKO)
  Char,
#endif
  End,
#if defined(CBINDGEN_IS_GECKO)
  MozCenter,
#endif
#if defined(CBINDGEN_IS_GECKO)
  MozLeft,
#endif
#if defined(CBINDGEN_IS_GECKO)
  MozRight,
#endif
#if defined(CBINDGEN_IS_SERVO)
  ServoCenter,
#endif
#if defined(CBINDGEN_IS_SERVO)
  ServoLeft,
#endif
#if defined(CBINDGEN_IS_SERVO)
  ServoRight,
#endif
};

/// Specified and computed value of text-align-last.
enum class StyleTextAlignLast : uint8_t {
  Auto,
  Start,
  End,
  Left,
  Right,
  Center,
  Justify,
};

/// Implements text-decoration-skip-ink which takes the keywords auto | none | all
///
/// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-ink-property
enum class StyleTextDecorationSkipInk : uint8_t {
  Auto,
  None,
  All,
};

/// Fill mode for the text-emphasis-style property
enum class StyleTextEmphasisFillMode : uint8_t {
  /// `filled`
  Filled,
  /// `open`
  Open,
};

/// Shape keyword for the text-emphasis-style property
enum class StyleTextEmphasisShapeKeyword : uint8_t {
  /// `dot`
  Dot,
  /// `circle`
  Circle,
  /// `double-circle`
  DoubleCircle,
  /// `triangle`
  Triangle,
  /// `sesame`
  Sesame,
};

/// Specified keyword values for case transforms in the text-transform property. (These are exclusive.)
enum class StyleTextTransformCase {
  /// No case transform.
  None,
  /// All uppercase.
  Uppercase,
  /// All lowercase.
  Lowercase,
  /// Capitalize each word.
  Capitalize,
};

enum class StyleTimingKeyword : uint8_t {
  Linear,
  Ease,
  EaseIn,
  EaseOut,
  EaseInOut,
};

enum class StyleTransformStyle : uint8_t {
  Flat,
  Preserve3d,
};

/// The specified value for the `user-select` property.
///
/// https://drafts.csswg.org/css-ui-4/#propdef-user-select
enum class StyleUserSelect : uint8_t {
  Auto,
  Text,
  None,
  /// Force selection of all children.
  All,
};

enum class StyleVerticalAlignKeyword : uint8_t {
  Baseline,
  Sub,
  Super,
  Top,
  TextTop,
  Middle,
  Bottom,
  TextBottom,
#if defined(CBINDGEN_IS_GECKO)
  MozMiddleWithBaseline,
#endif
};

/// A keyword for the Y direction.
enum class StyleVerticalPositionKeyword : uint8_t {
  Top,
  Bottom,
};

/// Values for the `word-break` property.
enum class StyleWordBreak : uint8_t {
  Normal,
  BreakAll,
  KeepAll,
#if defined(CBINDGEN_IS_GECKO)
  /// The break-word value, needed for compat.
  ///
  /// Specifying `word-break: break-word` makes `overflow-wrap` behave as
  /// `anywhere`, and `word-break` behave like `normal`.
  BreakWord,
#endif
};

#if defined(CBINDGEN_IS_GECKO)
/// Gecko-FFI-safe Arc (T is an ArcInner).
///
/// This can be null.
///
/// Leaks on drop. Please don't drop this.
template<typename GeckoType>
struct StyleStrong {
  const GeckoType *ptr;

  bool operator==(const StyleStrong& other) const {
    return ptr == other.ptr;
  }
  bool operator!=(const StyleStrong& other) const {
    return ptr != other.ptr;
  }
  already_AddRefed<GeckoType> Consume() {
    already_AddRefed<GeckoType> ret(const_cast<GeckoType*>(ptr));
    ptr = nullptr;
    return ret;
  }
};
#endif

/// A CSS float value.
using StyleCSSFloat = float;

/// A `<number>` value.
using StyleNumber = StyleCSSFloat;

/// A value of the `Scale` property
///
/// <https://drafts.csswg.org/css-transforms-2/#individual-transforms>
template<typename Number>
struct StyleGenericScale {
  enum class Tag : uint8_t {
    /// 'none'
    None,
    /// '<number>{1,3}'
    Scale,
  };

  struct StyleScale_Body {
    Number _0;
    Number _1;
    Number _2;

    bool operator==(const StyleScale_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1 &&
             _2 == other._2;
    }
    bool operator!=(const StyleScale_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1 ||
             _2 != other._2;
    }
  };

  Tag tag;
  union {
    StyleScale_Body scale;
  };

  static StyleGenericScale None() {
    StyleGenericScale result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericScale Scale(const Number &_0,
                                 const Number &_1,
                                 const Number &_2) {
    StyleGenericScale result;
    ::new (&result.scale._0) (Number)(_0);
    ::new (&result.scale._1) (Number)(_1);
    ::new (&result.scale._2) (Number)(_2);
    result.tag = Tag::Scale;
    return result;
  }

  bool IsScale() const {
    return tag == Tag::Scale;
  }

  const StyleScale_Body& AsScale() const {
    MOZ_DIAGNOSTIC_ASSERT(IsScale());
    return scale;
  }

  bool operator==(const StyleGenericScale& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Scale: return scale == other.scale;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericScale& other) const {
    return !(*this == other);
  }

  ~StyleGenericScale() {
    switch (tag) {
      case Tag::Scale: scale.~StyleScale_Body(); break;
      default: break;
    }
  }

  StyleGenericScale(const StyleGenericScale& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Scale: ::new (&scale) (StyleScale_Body)(other.scale); break;
      default: break;
    }
  }
  StyleGenericScale& operator=(const StyleGenericScale& other) {
    if (this != &other) {
      this->~StyleGenericScale();
      new (this) StyleGenericScale(other);
    }
    return *this;
  }
 public:
  // The implementation of IPC LayersMessages needs this to be public.
  StyleGenericScale(): tag(Tag::None) {}
};

/// A computed CSS `scale`
using StyleScale = StyleGenericScale<StyleNumber>;

/// The computed `<length>` value.
struct StyleCSSPixelLength {
  StyleCSSFloat _0;

  bool operator==(const StyleCSSPixelLength& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleCSSPixelLength& other) const {
    return _0 != other._0;
  }
  static StyleCSSPixelLength FromPixels(CSSCoord aCoord) { return {aCoord}; }
  static StyleCSSPixelLength Zero() { return FromPixels(0.0f); }

  inline nscoord ToAppUnits() const;
  inline bool IsZero() const;
  CSSCoord ToCSSPixels() const { return _0; }
  inline void ScaleBy(float);
  inline StyleCSSPixelLength ScaledBy(float) const;
};

/// An alias of computed `<length>` value.
using StyleLength = StyleCSSPixelLength;

struct StyleLengthVariant {
  uint8_t tag;
  StyleLength length;

  bool operator==(const StyleLengthVariant& other) const {
    return tag == other.tag &&
           length == other.length;
  }
  bool operator!=(const StyleLengthVariant& other) const {
    return tag != other.tag ||
           length != other.length;
  }
};

/// A computed percentage.
struct StylePercentage {
  StyleCSSFloat _0;

  bool operator==(const StylePercentage& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StylePercentage& other) const {
    return _0 != other._0;
  }
};

struct StylePercentageVariant {
  uint8_t tag;
  StylePercentage percentage;

  bool operator==(const StylePercentageVariant& other) const {
    return tag == other.tag &&
           percentage == other.percentage;
  }
  bool operator!=(const StylePercentageVariant& other) const {
    return tag != other.tag ||
           percentage != other.percentage;
  }
};

/// The leaves of a `<length-percentage>` calc expression.
union StyleCalcLengthPercentageLeaf {
  enum class Tag : uint8_t {
    Length,
    Percentage,
  };

  struct Length_Body {
    Tag tag;
    StyleLength _0;

    bool operator==(const Length_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Length_Body& other) const {
      return _0 != other._0;
    }
  };

  struct Percentage_Body {
    Tag tag;
    StylePercentage _0;

    bool operator==(const Percentage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Percentage_Body& other) const {
      return _0 != other._0;
    }
  };

  struct {
    Tag tag;
  };
  Length_Body length;
  Percentage_Body percentage;

  static StyleCalcLengthPercentageLeaf Length(const StyleLength &_0) {
    StyleCalcLengthPercentageLeaf result;
    ::new (&result.length._0) (StyleLength)(_0);
    result.tag = Tag::Length;
    return result;
  }

  bool IsLength() const {
    return tag == Tag::Length;
  }

  const StyleLength& AsLength() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLength());
    return length._0;
  }

  static StyleCalcLengthPercentageLeaf Percentage(const StylePercentage &_0) {
    StyleCalcLengthPercentageLeaf result;
    ::new (&result.percentage._0) (StylePercentage)(_0);
    result.tag = Tag::Percentage;
    return result;
  }

  bool IsPercentage() const {
    return tag == Tag::Percentage;
  }

  const StylePercentage& AsPercentage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPercentage());
    return percentage._0;
  }

  bool operator==(const StyleCalcLengthPercentageLeaf& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Length: return length == other.length;
      case Tag::Percentage: return percentage == other.percentage;

    }
    return true;
  }

  bool operator!=(const StyleCalcLengthPercentageLeaf& other) const {
    return !(*this == other);
  }

  private:
  StyleCalcLengthPercentageLeaf() {

  }
  public:


  ~StyleCalcLengthPercentageLeaf() {
    switch (tag) {
      case Tag::Length: length.~Length_Body(); break;
      case Tag::Percentage: percentage.~Percentage_Body(); break;

    }
  }

  StyleCalcLengthPercentageLeaf(const StyleCalcLengthPercentageLeaf& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Length: ::new (&length) (Length_Body)(other.length); break;
      case Tag::Percentage: ::new (&percentage) (Percentage_Body)(other.percentage); break;

    }
  }
  StyleCalcLengthPercentageLeaf& operator=(const StyleCalcLengthPercentageLeaf& other) {
    if (this != &other) {
      this->~StyleCalcLengthPercentageLeaf();
      new (this) StyleCalcLengthPercentageLeaf(other);
    }
    return *this;
  }
};

/// A struct that basically replaces a `Box<[T]>`, but which cbindgen can
/// understand.
///
/// We could rely on the struct layout of `Box<[T]>` per:
///
///   https://github.com/rust-lang/unsafe-code-guidelines/blob/master/reference/src/layout/pointers.md
///
/// But handling fat pointers with cbindgen both in structs and argument
/// positions more generally is a bit tricky.
///
template<typename T>
struct StyleOwnedSlice {
  T *ptr;
  uintptr_t len;
  StyleOwnedSlice() :
    ptr((T*)alignof(T)),
    len(0) {}

  inline void Clear();
  inline void CopyFrom(const StyleOwnedSlice&);
  inline void SwapElements(StyleOwnedSlice&);

  StyleOwnedSlice& operator=(const StyleOwnedSlice&);
  StyleOwnedSlice& operator=(StyleOwnedSlice&&);

  inline StyleOwnedSlice(const StyleOwnedSlice&);
  inline StyleOwnedSlice(StyleOwnedSlice&&);
  inline explicit StyleOwnedSlice(Vector<T>&&);

  inline ~StyleOwnedSlice();

  Span<const T> AsSpan() const {
    return {ptr, len};
  }

  size_t Length() const {
    return len;
  }

  bool IsEmpty() const { return Length() == 0; }

  bool operator==(const StyleOwnedSlice& other) const {
    return AsSpan() == other.AsSpan();
  }

  bool operator!=(const StyleOwnedSlice& other) const {
    return !(*this == other);
  }
};

/// A generic node in a calc expression.
///
/// FIXME: This would be much more elegant if we used `Self` in the types below,
/// but we can't because of https://github.com/serde-rs/serde/issues/1565.
///
/// FIXME: The following annotations are to workaround an LLVM inlining bug, see
/// bug 1631929.
///
template<typename L>
union StyleGenericCalcNode {
  enum class Tag : uint8_t {
    /// A leaf node.
    Leaf,
    /// A sum node, representing `a + b + c` where a, b, and c are the
    /// arguments.
    Sum,
    /// A `min` or `max` function.
    MinMax,
    /// A `clamp()` function.
    Clamp,
  };

  struct Leaf_Body {
    Tag tag;
    L _0;

    bool operator==(const Leaf_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Leaf_Body& other) const {
      return _0 != other._0;
    }
  };

  struct Sum_Body {
    Tag tag;
    StyleOwnedSlice<StyleGenericCalcNode<L>> _0;

    bool operator==(const Sum_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Sum_Body& other) const {
      return _0 != other._0;
    }
  };

  struct MinMax_Body {
    Tag tag;
    StyleOwnedSlice<StyleGenericCalcNode<L>> _0;
    StyleMinMaxOp _1;

    bool operator==(const MinMax_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const MinMax_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct Clamp_Body {
    Tag tag;
    /// The minimum value.
    StyleBox<StyleGenericCalcNode<L>> min;
    /// The central value.
    StyleBox<StyleGenericCalcNode<L>> center;
    /// The maximum value.
    StyleBox<StyleGenericCalcNode<L>> max;

    bool operator==(const Clamp_Body& other) const {
      return min == other.min &&
             center == other.center &&
             max == other.max;
    }
    bool operator!=(const Clamp_Body& other) const {
      return min != other.min ||
             center != other.center ||
             max != other.max;
    }
  };

  struct {
    Tag tag;
  };
  Leaf_Body leaf;
  Sum_Body sum;
  MinMax_Body min_max;
  Clamp_Body clamp;

  static StyleGenericCalcNode Leaf(const L &_0) {
    StyleGenericCalcNode result;
    ::new (&result.leaf._0) (L)(_0);
    result.tag = Tag::Leaf;
    return result;
  }

  bool IsLeaf() const {
    return tag == Tag::Leaf;
  }

  const L& AsLeaf() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLeaf());
    return leaf._0;
  }

  static StyleGenericCalcNode Sum(const StyleOwnedSlice<StyleGenericCalcNode<L>> &_0) {
    StyleGenericCalcNode result;
    ::new (&result.sum._0) (StyleOwnedSlice<StyleGenericCalcNode<L>>)(_0);
    result.tag = Tag::Sum;
    return result;
  }

  bool IsSum() const {
    return tag == Tag::Sum;
  }

  const StyleOwnedSlice<StyleGenericCalcNode<L>>& AsSum() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSum());
    return sum._0;
  }

  static StyleGenericCalcNode MinMax(const StyleOwnedSlice<StyleGenericCalcNode<L>> &_0,
                                     const StyleMinMaxOp &_1) {
    StyleGenericCalcNode result;
    ::new (&result.min_max._0) (StyleOwnedSlice<StyleGenericCalcNode<L>>)(_0);
    ::new (&result.min_max._1) (StyleMinMaxOp)(_1);
    result.tag = Tag::MinMax;
    return result;
  }

  bool IsMinMax() const {
    return tag == Tag::MinMax;
  }

  const MinMax_Body& AsMinMax() const {
    MOZ_DIAGNOSTIC_ASSERT(IsMinMax());
    return min_max;
  }

  static StyleGenericCalcNode Clamp(const StyleBox<StyleGenericCalcNode<L>> &min,
                                    const StyleBox<StyleGenericCalcNode<L>> &center,
                                    const StyleBox<StyleGenericCalcNode<L>> &max) {
    StyleGenericCalcNode result;
    ::new (&result.clamp.min) (StyleBox<StyleGenericCalcNode<L>>)(min);
    ::new (&result.clamp.center) (StyleBox<StyleGenericCalcNode<L>>)(center);
    ::new (&result.clamp.max) (StyleBox<StyleGenericCalcNode<L>>)(max);
    result.tag = Tag::Clamp;
    return result;
  }

  bool IsClamp() const {
    return tag == Tag::Clamp;
  }

  const Clamp_Body& AsClamp() const {
    MOZ_DIAGNOSTIC_ASSERT(IsClamp());
    return clamp;
  }

  MOZ_NEVER_INLINE bool operator==(const StyleGenericCalcNode& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Leaf: return leaf == other.leaf;
      case Tag::Sum: return sum == other.sum;
      case Tag::MinMax: return min_max == other.min_max;
      case Tag::Clamp: return clamp == other.clamp;

    }
    return true;
  }

  bool operator!=(const StyleGenericCalcNode& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericCalcNode() {

  }
  public:


  MOZ_NEVER_INLINE ~StyleGenericCalcNode() {
    switch (tag) {
      case Tag::Leaf: leaf.~Leaf_Body(); break;
      case Tag::Sum: sum.~Sum_Body(); break;
      case Tag::MinMax: min_max.~MinMax_Body(); break;
      case Tag::Clamp: clamp.~Clamp_Body(); break;

    }
  }

  MOZ_NEVER_INLINE StyleGenericCalcNode(const StyleGenericCalcNode& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Leaf: ::new (&leaf) (Leaf_Body)(other.leaf); break;
      case Tag::Sum: ::new (&sum) (Sum_Body)(other.sum); break;
      case Tag::MinMax: ::new (&min_max) (MinMax_Body)(other.min_max); break;
      case Tag::Clamp: ::new (&clamp) (Clamp_Body)(other.clamp); break;

    }
  }
  StyleGenericCalcNode& operator=(const StyleGenericCalcNode& other) {
    if (this != &other) {
      this->~StyleGenericCalcNode();
      new (this) StyleGenericCalcNode(other);
    }
    return *this;
  }
 private:

  template <typename ResultT, typename PercentageConverter>
  ResultT ResolveInternal(ResultT aPercentageBasis,
                          PercentageConverter aPercentageConverter) const;

 public:
  CSSCoord ResolveToCSSPixels(CSSCoord aPercentageBasis) const;

  using CoordPercentageRounder = nscoord(*)(float);
  nscoord Resolve(nscoord aBasis, CoordPercentageRounder) const;

  void ScaleLengthsBy(float);
};

/// The computed version of a calc() node for `<length-percentage>` values.
using StyleCalcNode = StyleGenericCalcNode<StyleCalcLengthPercentageLeaf>;

/// The representation of a calc() function with mixed lengths and percentages.
struct StyleCalcLengthPercentage {
  StyleAllowedNumericType clamping_mode;
  StyleCalcNode node;

  bool operator==(const StyleCalcLengthPercentage& other) const {
    return clamping_mode == other.clamping_mode &&
           node == other.node;
  }
  bool operator!=(const StyleCalcLengthPercentage& other) const {
    return clamping_mode != other.clamping_mode ||
           node != other.node;
  }
};

#if defined(SERVO_32_BITS)
struct StyleCalcVariant {
  uint8_t tag;
  StyleCalcLengthPercentage *ptr;

  bool operator==(const StyleCalcVariant& other) const {
    return tag == other.tag &&
           ptr == other.ptr;
  }
  bool operator!=(const StyleCalcVariant& other) const {
    return tag != other.tag ||
           ptr != other.ptr;
  }
};
#endif

#if defined(HAVE_64BIT_BUILD)
struct StyleCalcVariant {
  uintptr_t ptr;

  bool operator==(const StyleCalcVariant& other) const {
    return ptr == other.ptr;
  }
  bool operator!=(const StyleCalcVariant& other) const {
    return ptr != other.ptr;
  }
};
#endif

struct StyleTagVariant {
  uint8_t tag;

  bool operator==(const StyleTagVariant& other) const {
    return tag == other.tag;
  }
  bool operator!=(const StyleTagVariant& other) const {
    return tag != other.tag;
  }
};

union StyleLengthPercentageUnion {
  StyleLengthVariant length;
  StylePercentageVariant percentage;
  StyleCalcVariant calc;
  StyleTagVariant tag;
  using Self = StyleLengthPercentageUnion;

  // TODO(emilio): cbindgen should be able to generate these in the body of the
  // union, but it seems it's only implemented for structs, not unions.
  static const uint8_t TAG_CALC = StyleLengthPercentageUnion_TAG_CALC;
  static const uint8_t TAG_LENGTH = StyleLengthPercentageUnion_TAG_LENGTH;
  static const uint8_t TAG_PERCENTAGE = StyleLengthPercentageUnion_TAG_PERCENTAGE;
  static const uint8_t TAG_MASK = StyleLengthPercentageUnion_TAG_MASK;

 private:
  uint8_t Tag() const {
    return tag.tag & TAG_MASK;
  }

 public:
  // We need to do all this manually because cbingen can't reason about unions.
  inline StyleLengthPercentageUnion();
  inline StyleLengthPercentageUnion(const Self&);
  inline ~StyleLengthPercentageUnion();
  inline Self& operator=(const Self&);

  inline bool operator==(const Self& aOther) const;
  inline bool operator!=(const Self& aOther) const;

  inline bool IsLength() const;
  inline bool IsPercentage() const;
  inline bool IsCalc() const;

  inline const StyleLength& AsLength() const;
  inline StyleLength& AsLength();

  inline const StylePercentage& AsPercentage() const;
  inline StylePercentage& AsPercentage();

  inline const StyleCalcLengthPercentage& AsCalc() const;
  inline StyleCalcLengthPercentage& AsCalc();

  static inline Self Zero();
  static inline Self FromAppUnits(nscoord);
  static inline Self FromPixels(CSSCoord);
  static inline Self FromPercentage(float);

  inline void ScaleLengthsBy(float);
  inline bool HasPercent() const;
  inline bool ConvertsToLength() const;
  inline nscoord ToLength() const;
  inline CSSCoord ToLengthInCSSPixels() const;
  inline bool ConvertsToPercentage() const;
  inline bool HasLengthAndPercentage() const;
  inline float ToPercentage() const;
  inline bool IsDefinitelyZero() const;
  inline CSSCoord ResolveToCSSPixels(CSSCoord aPercentageBasisInCSSPixels) const;
  template<typename T> inline CSSCoord ResolveToCSSPixelsWith(T aPercentageGetter) const;
  template<typename T, typename U>
  inline nscoord Resolve(T aPercentageGetter, U aPercentRoundingFunction) const;
  template<typename T>
  inline nscoord Resolve(nscoord aPercentageBasis, T aPercentRoundingFunction) const;
  template<typename T> inline nscoord Resolve(T aPercentageGetter) const;
  inline nscoord Resolve(nscoord aPercentageBasis) const;
};

/// A `<length-percentage>` value. This can be either a `<length>`, a
/// `<percentage>`, or a combination of both via `calc()`.
///
///
/// https://drafts.csswg.org/css-values-4/#typedef-length-percentage
///
/// The tag is stored in the lower two bits.
///
/// We need to use a struct instead of the union directly because unions with
/// Drop implementations are unstable, looks like.
///
/// Also we need the union and the variants to be `pub` (even though the member
/// is private) so that cbindgen generates it. They're not part of the public
/// API otherwise.
using StyleLengthPercentage = StyleLengthPercentageUnion;

/// A value of the `translate` property
///
/// https://drafts.csswg.org/css-transforms-2/#individual-transform-serialization:
///
/// If a 2d translation is specified, the property must serialize with only one
/// or two values (per usual, if the second value is 0px, the default, it must
/// be omitted when serializing).
///
/// If a 3d translation is specified and the value can be expressed as 2d, we treat as 2d and
/// serialize accoringly. Otherwise, we serialize all three values.
/// https://github.com/w3c/csswg-drafts/issues/3305
///
/// <https://drafts.csswg.org/css-transforms-2/#individual-transforms>
template<typename LengthPercentage, typename Length>
struct StyleGenericTranslate {
  enum class Tag : uint8_t {
    /// 'none'
    None,
    /// <length-percentage> [ <length-percentage> <length>? ]?
    Translate,
  };

  struct StyleTranslate_Body {
    LengthPercentage _0;
    LengthPercentage _1;
    Length _2;

    bool operator==(const StyleTranslate_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1 &&
             _2 == other._2;
    }
    bool operator!=(const StyleTranslate_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1 ||
             _2 != other._2;
    }
  };

  Tag tag;
  union {
    StyleTranslate_Body translate;
  };

  static StyleGenericTranslate None() {
    StyleGenericTranslate result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericTranslate Translate(const LengthPercentage &_0,
                                         const LengthPercentage &_1,
                                         const Length &_2) {
    StyleGenericTranslate result;
    ::new (&result.translate._0) (LengthPercentage)(_0);
    ::new (&result.translate._1) (LengthPercentage)(_1);
    ::new (&result.translate._2) (Length)(_2);
    result.tag = Tag::Translate;
    return result;
  }

  bool IsTranslate() const {
    return tag == Tag::Translate;
  }

  const StyleTranslate_Body& AsTranslate() const {
    MOZ_DIAGNOSTIC_ASSERT(IsTranslate());
    return translate;
  }

  bool operator==(const StyleGenericTranslate& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Translate: return translate == other.translate;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericTranslate& other) const {
    return !(*this == other);
  }

  ~StyleGenericTranslate() {
    switch (tag) {
      case Tag::Translate: translate.~StyleTranslate_Body(); break;
      default: break;
    }
  }

  StyleGenericTranslate(const StyleGenericTranslate& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Translate: ::new (&translate) (StyleTranslate_Body)(other.translate); break;
      default: break;
    }
  }
  StyleGenericTranslate& operator=(const StyleGenericTranslate& other) {
    if (this != &other) {
      this->~StyleGenericTranslate();
      new (this) StyleGenericTranslate(other);
    }
    return *this;
  }
 public:
  // The implementation of IPC LayersMessages needs this to be public.
  StyleGenericTranslate(): tag(Tag::None) {}
};

/// A computed CSS `translate`
using StyleTranslate = StyleGenericTranslate<StyleLengthPercentage, StyleLength>;

/// A computed angle in degrees.
struct StyleAngle {
  StyleCSSFloat _0;

  bool operator==(const StyleAngle& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleAngle& other) const {
    return _0 != other._0;
  }
  inline static StyleAngle Zero();
  inline float ToDegrees() const;
  inline double ToRadians() const;
  StyleAngle operator+(const StyleAngle& aAngle) const {
    return StyleAngle{_0 + aAngle._0};
  }
  StyleAngle operator-(const StyleAngle& aAngle) const {
    return StyleAngle{_0 - aAngle._0};
  }
};

/// A value of the `Rotate` property
///
/// <https://drafts.csswg.org/css-transforms-2/#individual-transforms>
template<typename Number, typename Angle>
struct StyleGenericRotate {
  enum class Tag : uint8_t {
    /// 'none'
    None,
    /// '<angle>'
    Rotate,
    /// '<number>{3} <angle>'
    Rotate3D,
  };

  struct StyleRotate_Body {
    Angle _0;

    bool operator==(const StyleRotate_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleRotate_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleRotate3D_Body {
    Number _0;
    Number _1;
    Number _2;
    Angle _3;

    bool operator==(const StyleRotate3D_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1 &&
             _2 == other._2 &&
             _3 == other._3;
    }
    bool operator!=(const StyleRotate3D_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1 ||
             _2 != other._2 ||
             _3 != other._3;
    }
  };

  Tag tag;
  union {
    StyleRotate_Body rotate;
    StyleRotate3D_Body rotate3_d;
  };

  static StyleGenericRotate None() {
    StyleGenericRotate result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericRotate Rotate(const Angle &_0) {
    StyleGenericRotate result;
    ::new (&result.rotate._0) (Angle)(_0);
    result.tag = Tag::Rotate;
    return result;
  }

  bool IsRotate() const {
    return tag == Tag::Rotate;
  }

  const Angle& AsRotate() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRotate());
    return rotate._0;
  }

  static StyleGenericRotate Rotate3D(const Number &_0,
                                     const Number &_1,
                                     const Number &_2,
                                     const Angle &_3) {
    StyleGenericRotate result;
    ::new (&result.rotate3_d._0) (Number)(_0);
    ::new (&result.rotate3_d._1) (Number)(_1);
    ::new (&result.rotate3_d._2) (Number)(_2);
    ::new (&result.rotate3_d._3) (Angle)(_3);
    result.tag = Tag::Rotate3D;
    return result;
  }

  bool IsRotate3D() const {
    return tag == Tag::Rotate3D;
  }

  const StyleRotate3D_Body& AsRotate3D() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRotate3D());
    return rotate3_d;
  }

  bool operator==(const StyleGenericRotate& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Rotate: return rotate == other.rotate;
      case Tag::Rotate3D: return rotate3_d == other.rotate3_d;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericRotate& other) const {
    return !(*this == other);
  }

  ~StyleGenericRotate() {
    switch (tag) {
      case Tag::Rotate: rotate.~StyleRotate_Body(); break;
      case Tag::Rotate3D: rotate3_d.~StyleRotate3D_Body(); break;
      default: break;
    }
  }

  StyleGenericRotate(const StyleGenericRotate& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Rotate: ::new (&rotate) (StyleRotate_Body)(other.rotate); break;
      case Tag::Rotate3D: ::new (&rotate3_d) (StyleRotate3D_Body)(other.rotate3_d); break;
      default: break;
    }
  }
  StyleGenericRotate& operator=(const StyleGenericRotate& other) {
    if (this != &other) {
      this->~StyleGenericRotate();
      new (this) StyleGenericRotate(other);
    }
    return *this;
  }
 public:
  // The implementation of IPC LayersMessages needs this to be public.
  StyleGenericRotate(): tag(Tag::None) {}
};

/// A computed CSS `rotate`
using StyleRotate = StyleGenericRotate<StyleNumber, StyleAngle>;

/// A CSS integer value.
using StyleCSSInteger = int32_t;

/// A `<integer>` value.
using StyleInteger = StyleCSSInteger;

/// A generic 2D transformation matrix.
template<typename T>
struct StyleGenericMatrix {
  T a;
  T b;
  T c;
  T d;
  T e;
  T f;

  bool operator==(const StyleGenericMatrix& other) const {
    return a == other.a &&
           b == other.b &&
           c == other.c &&
           d == other.d &&
           e == other.e &&
           f == other.f;
  }
  bool operator!=(const StyleGenericMatrix& other) const {
    return a != other.a ||
           b != other.b ||
           c != other.c ||
           d != other.d ||
           e != other.e ||
           f != other.f;
  }
};

template<typename T>
struct StyleGenericMatrix3D {
  T m11;
  T m12;
  T m13;
  T m14;
  T m21;
  T m22;
  T m23;
  T m24;
  T m31;
  T m32;
  T m33;
  T m34;
  T m41;
  T m42;
  T m43;
  T m44;

  bool operator==(const StyleGenericMatrix3D& other) const {
    return m11 == other.m11 &&
           m12 == other.m12 &&
           m13 == other.m13 &&
           m14 == other.m14 &&
           m21 == other.m21 &&
           m22 == other.m22 &&
           m23 == other.m23 &&
           m24 == other.m24 &&
           m31 == other.m31 &&
           m32 == other.m32 &&
           m33 == other.m33 &&
           m34 == other.m34 &&
           m41 == other.m41 &&
           m42 == other.m42 &&
           m43 == other.m43 &&
           m44 == other.m44;
  }
  bool operator!=(const StyleGenericMatrix3D& other) const {
    return m11 != other.m11 ||
           m12 != other.m12 ||
           m13 != other.m13 ||
           m14 != other.m14 ||
           m21 != other.m21 ||
           m22 != other.m22 ||
           m23 != other.m23 ||
           m24 != other.m24 ||
           m31 != other.m31 ||
           m32 != other.m32 ||
           m33 != other.m33 ||
           m34 != other.m34 ||
           m41 != other.m41 ||
           m42 != other.m42 ||
           m43 != other.m43 ||
           m44 != other.m44;
  }
};

/// A value of the `transform` property
template<typename T>
struct StyleGenericTransform {
  StyleOwnedSlice<T> _0;

  bool operator==(const StyleGenericTransform& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleGenericTransform& other) const {
    return _0 != other._0;
  }
  inline Span<const T> Operations() const;
  inline bool IsNone() const;
  bool HasPercent() const;
};

/// A single operation in the list of a `transform` value
template<typename Angle, typename Number, typename Length, typename Integer, typename LengthPercentage>
struct StyleGenericTransformOperation {
  enum class Tag : uint8_t {
    /// Represents a 2D 2x3 matrix.
    Matrix,
    /// Represents a 3D 4x4 matrix.
    Matrix3D,
    /// A 2D skew.
    ///
    /// If the second angle is not provided it is assumed zero.
    ///
    /// Syntax can be skew(angle) or skew(angle, angle)
    Skew,
    /// skewX(angle)
    SkewX,
    /// skewY(angle)
    SkewY,
    /// translate(x, y) or translate(x)
    Translate,
    /// translateX(x)
    TranslateX,
    /// translateY(y)
    TranslateY,
    /// translateZ(z)
    TranslateZ,
    /// translate3d(x, y, z)
    Translate3D,
    /// A 2D scaling factor.
    ///
    /// Syntax can be scale(factor) or scale(factor, factor)
    Scale,
    /// scaleX(factor)
    ScaleX,
    /// scaleY(factor)
    ScaleY,
    /// scaleZ(factor)
    ScaleZ,
    /// scale3D(factorX, factorY, factorZ)
    Scale3D,
    /// Describes a 2D Rotation.
    ///
    /// In a 3D scene `rotate(angle)` is equivalent to `rotateZ(angle)`.
    Rotate,
    /// Rotation in 3D space around the x-axis.
    RotateX,
    /// Rotation in 3D space around the y-axis.
    RotateY,
    /// Rotation in 3D space around the z-axis.
    RotateZ,
    /// Rotation in 3D space.
    ///
    /// Generalization of rotateX, rotateY and rotateZ.
    Rotate3D,
    /// Specifies a perspective projection matrix.
    ///
    /// Part of CSS Transform Module Level 2 and defined at
    /// [§ 13.1. 3D Transform Function](https://drafts.csswg.org/css-transforms-2/#funcdef-perspective).
    ///
    /// The value must be greater than or equal to zero.
    Perspective,
    /// A intermediate type for interpolation of mismatched transform lists.
    InterpolateMatrix,
    /// A intermediate type for accumulation of mismatched transform lists.
    AccumulateMatrix,
  };

  struct StyleMatrix_Body {
    StyleGenericMatrix<Number> _0;

    bool operator==(const StyleMatrix_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleMatrix_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleMatrix3D_Body {
    StyleGenericMatrix3D<Number> _0;

    bool operator==(const StyleMatrix3D_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleMatrix3D_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleSkew_Body {
    Angle _0;
    Angle _1;

    bool operator==(const StyleSkew_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const StyleSkew_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct StyleSkewX_Body {
    Angle _0;

    bool operator==(const StyleSkewX_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleSkewX_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleSkewY_Body {
    Angle _0;

    bool operator==(const StyleSkewY_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleSkewY_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleTranslate_Body {
    LengthPercentage _0;
    LengthPercentage _1;

    bool operator==(const StyleTranslate_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const StyleTranslate_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct StyleTranslateX_Body {
    LengthPercentage _0;

    bool operator==(const StyleTranslateX_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleTranslateX_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleTranslateY_Body {
    LengthPercentage _0;

    bool operator==(const StyleTranslateY_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleTranslateY_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleTranslateZ_Body {
    Length _0;

    bool operator==(const StyleTranslateZ_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleTranslateZ_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleTranslate3D_Body {
    LengthPercentage _0;
    LengthPercentage _1;
    Length _2;

    bool operator==(const StyleTranslate3D_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1 &&
             _2 == other._2;
    }
    bool operator!=(const StyleTranslate3D_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1 ||
             _2 != other._2;
    }
  };

  struct StyleScale_Body {
    Number _0;
    Number _1;

    bool operator==(const StyleScale_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const StyleScale_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct StyleScaleX_Body {
    Number _0;

    bool operator==(const StyleScaleX_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleScaleX_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleScaleY_Body {
    Number _0;

    bool operator==(const StyleScaleY_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleScaleY_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleScaleZ_Body {
    Number _0;

    bool operator==(const StyleScaleZ_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleScaleZ_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleScale3D_Body {
    Number _0;
    Number _1;
    Number _2;

    bool operator==(const StyleScale3D_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1 &&
             _2 == other._2;
    }
    bool operator!=(const StyleScale3D_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1 ||
             _2 != other._2;
    }
  };

  struct StyleRotate_Body {
    Angle _0;

    bool operator==(const StyleRotate_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleRotate_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleRotateX_Body {
    Angle _0;

    bool operator==(const StyleRotateX_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleRotateX_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleRotateY_Body {
    Angle _0;

    bool operator==(const StyleRotateY_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleRotateY_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleRotateZ_Body {
    Angle _0;

    bool operator==(const StyleRotateZ_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleRotateZ_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleRotate3D_Body {
    Number _0;
    Number _1;
    Number _2;
    Angle _3;

    bool operator==(const StyleRotate3D_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1 &&
             _2 == other._2 &&
             _3 == other._3;
    }
    bool operator!=(const StyleRotate3D_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1 ||
             _2 != other._2 ||
             _3 != other._3;
    }
  };

  struct StylePerspective_Body {
    Length _0;

    bool operator==(const StylePerspective_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StylePerspective_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleInterpolateMatrix_Body {
    StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>> from_list;
    StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>> to_list;
    StylePercentage progress;

    bool operator==(const StyleInterpolateMatrix_Body& other) const {
      return from_list == other.from_list &&
             to_list == other.to_list &&
             progress == other.progress;
    }
    bool operator!=(const StyleInterpolateMatrix_Body& other) const {
      return from_list != other.from_list ||
             to_list != other.to_list ||
             progress != other.progress;
    }
  };

  struct StyleAccumulateMatrix_Body {
    StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>> from_list;
    StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>> to_list;
    Integer count;

    bool operator==(const StyleAccumulateMatrix_Body& other) const {
      return from_list == other.from_list &&
             to_list == other.to_list &&
             count == other.count;
    }
    bool operator!=(const StyleAccumulateMatrix_Body& other) const {
      return from_list != other.from_list ||
             to_list != other.to_list ||
             count != other.count;
    }
  };

  Tag tag;
  union {
    StyleMatrix_Body matrix;
    StyleMatrix3D_Body matrix3_d;
    StyleSkew_Body skew;
    StyleSkewX_Body skew_x;
    StyleSkewY_Body skew_y;
    StyleTranslate_Body translate;
    StyleTranslateX_Body translate_x;
    StyleTranslateY_Body translate_y;
    StyleTranslateZ_Body translate_z;
    StyleTranslate3D_Body translate3_d;
    StyleScale_Body scale;
    StyleScaleX_Body scale_x;
    StyleScaleY_Body scale_y;
    StyleScaleZ_Body scale_z;
    StyleScale3D_Body scale3_d;
    StyleRotate_Body rotate;
    StyleRotateX_Body rotate_x;
    StyleRotateY_Body rotate_y;
    StyleRotateZ_Body rotate_z;
    StyleRotate3D_Body rotate3_d;
    StylePerspective_Body perspective;
    StyleInterpolateMatrix_Body interpolate_matrix;
    StyleAccumulateMatrix_Body accumulate_matrix;
  };

  static StyleGenericTransformOperation Matrix(const StyleGenericMatrix<Number> &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.matrix._0) (StyleGenericMatrix<Number>)(_0);
    result.tag = Tag::Matrix;
    return result;
  }

  bool IsMatrix() const {
    return tag == Tag::Matrix;
  }

  const StyleGenericMatrix<Number>& AsMatrix() const {
    MOZ_DIAGNOSTIC_ASSERT(IsMatrix());
    return matrix._0;
  }

  static StyleGenericTransformOperation Matrix3D(const StyleGenericMatrix3D<Number> &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.matrix3_d._0) (StyleGenericMatrix3D<Number>)(_0);
    result.tag = Tag::Matrix3D;
    return result;
  }

  bool IsMatrix3D() const {
    return tag == Tag::Matrix3D;
  }

  const StyleGenericMatrix3D<Number>& AsMatrix3D() const {
    MOZ_DIAGNOSTIC_ASSERT(IsMatrix3D());
    return matrix3_d._0;
  }

  static StyleGenericTransformOperation Skew(const Angle &_0,
                                             const Angle &_1) {
    StyleGenericTransformOperation result;
    ::new (&result.skew._0) (Angle)(_0);
    ::new (&result.skew._1) (Angle)(_1);
    result.tag = Tag::Skew;
    return result;
  }

  bool IsSkew() const {
    return tag == Tag::Skew;
  }

  const StyleSkew_Body& AsSkew() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSkew());
    return skew;
  }

  static StyleGenericTransformOperation SkewX(const Angle &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.skew_x._0) (Angle)(_0);
    result.tag = Tag::SkewX;
    return result;
  }

  bool IsSkewX() const {
    return tag == Tag::SkewX;
  }

  const Angle& AsSkewX() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSkewX());
    return skew_x._0;
  }

  static StyleGenericTransformOperation SkewY(const Angle &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.skew_y._0) (Angle)(_0);
    result.tag = Tag::SkewY;
    return result;
  }

  bool IsSkewY() const {
    return tag == Tag::SkewY;
  }

  const Angle& AsSkewY() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSkewY());
    return skew_y._0;
  }

  static StyleGenericTransformOperation Translate(const LengthPercentage &_0,
                                                  const LengthPercentage &_1) {
    StyleGenericTransformOperation result;
    ::new (&result.translate._0) (LengthPercentage)(_0);
    ::new (&result.translate._1) (LengthPercentage)(_1);
    result.tag = Tag::Translate;
    return result;
  }

  bool IsTranslate() const {
    return tag == Tag::Translate;
  }

  const StyleTranslate_Body& AsTranslate() const {
    MOZ_DIAGNOSTIC_ASSERT(IsTranslate());
    return translate;
  }

  static StyleGenericTransformOperation TranslateX(const LengthPercentage &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.translate_x._0) (LengthPercentage)(_0);
    result.tag = Tag::TranslateX;
    return result;
  }

  bool IsTranslateX() const {
    return tag == Tag::TranslateX;
  }

  const LengthPercentage& AsTranslateX() const {
    MOZ_DIAGNOSTIC_ASSERT(IsTranslateX());
    return translate_x._0;
  }

  static StyleGenericTransformOperation TranslateY(const LengthPercentage &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.translate_y._0) (LengthPercentage)(_0);
    result.tag = Tag::TranslateY;
    return result;
  }

  bool IsTranslateY() const {
    return tag == Tag::TranslateY;
  }

  const LengthPercentage& AsTranslateY() const {
    MOZ_DIAGNOSTIC_ASSERT(IsTranslateY());
    return translate_y._0;
  }

  static StyleGenericTransformOperation TranslateZ(const Length &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.translate_z._0) (Length)(_0);
    result.tag = Tag::TranslateZ;
    return result;
  }

  bool IsTranslateZ() const {
    return tag == Tag::TranslateZ;
  }

  const Length& AsTranslateZ() const {
    MOZ_DIAGNOSTIC_ASSERT(IsTranslateZ());
    return translate_z._0;
  }

  static StyleGenericTransformOperation Translate3D(const LengthPercentage &_0,
                                                    const LengthPercentage &_1,
                                                    const Length &_2) {
    StyleGenericTransformOperation result;
    ::new (&result.translate3_d._0) (LengthPercentage)(_0);
    ::new (&result.translate3_d._1) (LengthPercentage)(_1);
    ::new (&result.translate3_d._2) (Length)(_2);
    result.tag = Tag::Translate3D;
    return result;
  }

  bool IsTranslate3D() const {
    return tag == Tag::Translate3D;
  }

  const StyleTranslate3D_Body& AsTranslate3D() const {
    MOZ_DIAGNOSTIC_ASSERT(IsTranslate3D());
    return translate3_d;
  }

  static StyleGenericTransformOperation Scale(const Number &_0,
                                              const Number &_1) {
    StyleGenericTransformOperation result;
    ::new (&result.scale._0) (Number)(_0);
    ::new (&result.scale._1) (Number)(_1);
    result.tag = Tag::Scale;
    return result;
  }

  bool IsScale() const {
    return tag == Tag::Scale;
  }

  const StyleScale_Body& AsScale() const {
    MOZ_DIAGNOSTIC_ASSERT(IsScale());
    return scale;
  }

  static StyleGenericTransformOperation ScaleX(const Number &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.scale_x._0) (Number)(_0);
    result.tag = Tag::ScaleX;
    return result;
  }

  bool IsScaleX() const {
    return tag == Tag::ScaleX;
  }

  const Number& AsScaleX() const {
    MOZ_DIAGNOSTIC_ASSERT(IsScaleX());
    return scale_x._0;
  }

  static StyleGenericTransformOperation ScaleY(const Number &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.scale_y._0) (Number)(_0);
    result.tag = Tag::ScaleY;
    return result;
  }

  bool IsScaleY() const {
    return tag == Tag::ScaleY;
  }

  const Number& AsScaleY() const {
    MOZ_DIAGNOSTIC_ASSERT(IsScaleY());
    return scale_y._0;
  }

  static StyleGenericTransformOperation ScaleZ(const Number &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.scale_z._0) (Number)(_0);
    result.tag = Tag::ScaleZ;
    return result;
  }

  bool IsScaleZ() const {
    return tag == Tag::ScaleZ;
  }

  const Number& AsScaleZ() const {
    MOZ_DIAGNOSTIC_ASSERT(IsScaleZ());
    return scale_z._0;
  }

  static StyleGenericTransformOperation Scale3D(const Number &_0,
                                                const Number &_1,
                                                const Number &_2) {
    StyleGenericTransformOperation result;
    ::new (&result.scale3_d._0) (Number)(_0);
    ::new (&result.scale3_d._1) (Number)(_1);
    ::new (&result.scale3_d._2) (Number)(_2);
    result.tag = Tag::Scale3D;
    return result;
  }

  bool IsScale3D() const {
    return tag == Tag::Scale3D;
  }

  const StyleScale3D_Body& AsScale3D() const {
    MOZ_DIAGNOSTIC_ASSERT(IsScale3D());
    return scale3_d;
  }

  static StyleGenericTransformOperation Rotate(const Angle &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.rotate._0) (Angle)(_0);
    result.tag = Tag::Rotate;
    return result;
  }

  bool IsRotate() const {
    return tag == Tag::Rotate;
  }

  const Angle& AsRotate() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRotate());
    return rotate._0;
  }

  static StyleGenericTransformOperation RotateX(const Angle &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.rotate_x._0) (Angle)(_0);
    result.tag = Tag::RotateX;
    return result;
  }

  bool IsRotateX() const {
    return tag == Tag::RotateX;
  }

  const Angle& AsRotateX() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRotateX());
    return rotate_x._0;
  }

  static StyleGenericTransformOperation RotateY(const Angle &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.rotate_y._0) (Angle)(_0);
    result.tag = Tag::RotateY;
    return result;
  }

  bool IsRotateY() const {
    return tag == Tag::RotateY;
  }

  const Angle& AsRotateY() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRotateY());
    return rotate_y._0;
  }

  static StyleGenericTransformOperation RotateZ(const Angle &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.rotate_z._0) (Angle)(_0);
    result.tag = Tag::RotateZ;
    return result;
  }

  bool IsRotateZ() const {
    return tag == Tag::RotateZ;
  }

  const Angle& AsRotateZ() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRotateZ());
    return rotate_z._0;
  }

  static StyleGenericTransformOperation Rotate3D(const Number &_0,
                                                 const Number &_1,
                                                 const Number &_2,
                                                 const Angle &_3) {
    StyleGenericTransformOperation result;
    ::new (&result.rotate3_d._0) (Number)(_0);
    ::new (&result.rotate3_d._1) (Number)(_1);
    ::new (&result.rotate3_d._2) (Number)(_2);
    ::new (&result.rotate3_d._3) (Angle)(_3);
    result.tag = Tag::Rotate3D;
    return result;
  }

  bool IsRotate3D() const {
    return tag == Tag::Rotate3D;
  }

  const StyleRotate3D_Body& AsRotate3D() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRotate3D());
    return rotate3_d;
  }

  static StyleGenericTransformOperation Perspective(const Length &_0) {
    StyleGenericTransformOperation result;
    ::new (&result.perspective._0) (Length)(_0);
    result.tag = Tag::Perspective;
    return result;
  }

  bool IsPerspective() const {
    return tag == Tag::Perspective;
  }

  const Length& AsPerspective() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPerspective());
    return perspective._0;
  }

  static StyleGenericTransformOperation InterpolateMatrix(const StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>> &from_list,
                                                          const StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>> &to_list,
                                                          const StylePercentage &progress) {
    StyleGenericTransformOperation result;
    ::new (&result.interpolate_matrix.from_list) (StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>>)(from_list);
    ::new (&result.interpolate_matrix.to_list) (StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>>)(to_list);
    ::new (&result.interpolate_matrix.progress) (StylePercentage)(progress);
    result.tag = Tag::InterpolateMatrix;
    return result;
  }

  bool IsInterpolateMatrix() const {
    return tag == Tag::InterpolateMatrix;
  }

  const StyleInterpolateMatrix_Body& AsInterpolateMatrix() const {
    MOZ_DIAGNOSTIC_ASSERT(IsInterpolateMatrix());
    return interpolate_matrix;
  }

  static StyleGenericTransformOperation AccumulateMatrix(const StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>> &from_list,
                                                         const StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>> &to_list,
                                                         const Integer &count) {
    StyleGenericTransformOperation result;
    ::new (&result.accumulate_matrix.from_list) (StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>>)(from_list);
    ::new (&result.accumulate_matrix.to_list) (StyleGenericTransform<StyleGenericTransformOperation<Angle, Number, Length, Integer, LengthPercentage>>)(to_list);
    ::new (&result.accumulate_matrix.count) (Integer)(count);
    result.tag = Tag::AccumulateMatrix;
    return result;
  }

  bool IsAccumulateMatrix() const {
    return tag == Tag::AccumulateMatrix;
  }

  const StyleAccumulateMatrix_Body& AsAccumulateMatrix() const {
    MOZ_DIAGNOSTIC_ASSERT(IsAccumulateMatrix());
    return accumulate_matrix;
  }

  bool operator==(const StyleGenericTransformOperation& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Matrix: return matrix == other.matrix;
      case Tag::Matrix3D: return matrix3_d == other.matrix3_d;
      case Tag::Skew: return skew == other.skew;
      case Tag::SkewX: return skew_x == other.skew_x;
      case Tag::SkewY: return skew_y == other.skew_y;
      case Tag::Translate: return translate == other.translate;
      case Tag::TranslateX: return translate_x == other.translate_x;
      case Tag::TranslateY: return translate_y == other.translate_y;
      case Tag::TranslateZ: return translate_z == other.translate_z;
      case Tag::Translate3D: return translate3_d == other.translate3_d;
      case Tag::Scale: return scale == other.scale;
      case Tag::ScaleX: return scale_x == other.scale_x;
      case Tag::ScaleY: return scale_y == other.scale_y;
      case Tag::ScaleZ: return scale_z == other.scale_z;
      case Tag::Scale3D: return scale3_d == other.scale3_d;
      case Tag::Rotate: return rotate == other.rotate;
      case Tag::RotateX: return rotate_x == other.rotate_x;
      case Tag::RotateY: return rotate_y == other.rotate_y;
      case Tag::RotateZ: return rotate_z == other.rotate_z;
      case Tag::Rotate3D: return rotate3_d == other.rotate3_d;
      case Tag::Perspective: return perspective == other.perspective;
      case Tag::InterpolateMatrix: return interpolate_matrix == other.interpolate_matrix;
      case Tag::AccumulateMatrix: return accumulate_matrix == other.accumulate_matrix;

    }
    return true;
  }

  bool operator!=(const StyleGenericTransformOperation& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericTransformOperation() {

  }
  public:


  ~StyleGenericTransformOperation() {
    switch (tag) {
      case Tag::Matrix: matrix.~StyleMatrix_Body(); break;
      case Tag::Matrix3D: matrix3_d.~StyleMatrix3D_Body(); break;
      case Tag::Skew: skew.~StyleSkew_Body(); break;
      case Tag::SkewX: skew_x.~StyleSkewX_Body(); break;
      case Tag::SkewY: skew_y.~StyleSkewY_Body(); break;
      case Tag::Translate: translate.~StyleTranslate_Body(); break;
      case Tag::TranslateX: translate_x.~StyleTranslateX_Body(); break;
      case Tag::TranslateY: translate_y.~StyleTranslateY_Body(); break;
      case Tag::TranslateZ: translate_z.~StyleTranslateZ_Body(); break;
      case Tag::Translate3D: translate3_d.~StyleTranslate3D_Body(); break;
      case Tag::Scale: scale.~StyleScale_Body(); break;
      case Tag::ScaleX: scale_x.~StyleScaleX_Body(); break;
      case Tag::ScaleY: scale_y.~StyleScaleY_Body(); break;
      case Tag::ScaleZ: scale_z.~StyleScaleZ_Body(); break;
      case Tag::Scale3D: scale3_d.~StyleScale3D_Body(); break;
      case Tag::Rotate: rotate.~StyleRotate_Body(); break;
      case Tag::RotateX: rotate_x.~StyleRotateX_Body(); break;
      case Tag::RotateY: rotate_y.~StyleRotateY_Body(); break;
      case Tag::RotateZ: rotate_z.~StyleRotateZ_Body(); break;
      case Tag::Rotate3D: rotate3_d.~StyleRotate3D_Body(); break;
      case Tag::Perspective: perspective.~StylePerspective_Body(); break;
      case Tag::InterpolateMatrix: interpolate_matrix.~StyleInterpolateMatrix_Body(); break;
      case Tag::AccumulateMatrix: accumulate_matrix.~StyleAccumulateMatrix_Body(); break;

    }
  }

  StyleGenericTransformOperation(const StyleGenericTransformOperation& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Matrix: ::new (&matrix) (StyleMatrix_Body)(other.matrix); break;
      case Tag::Matrix3D: ::new (&matrix3_d) (StyleMatrix3D_Body)(other.matrix3_d); break;
      case Tag::Skew: ::new (&skew) (StyleSkew_Body)(other.skew); break;
      case Tag::SkewX: ::new (&skew_x) (StyleSkewX_Body)(other.skew_x); break;
      case Tag::SkewY: ::new (&skew_y) (StyleSkewY_Body)(other.skew_y); break;
      case Tag::Translate: ::new (&translate) (StyleTranslate_Body)(other.translate); break;
      case Tag::TranslateX: ::new (&translate_x) (StyleTranslateX_Body)(other.translate_x); break;
      case Tag::TranslateY: ::new (&translate_y) (StyleTranslateY_Body)(other.translate_y); break;
      case Tag::TranslateZ: ::new (&translate_z) (StyleTranslateZ_Body)(other.translate_z); break;
      case Tag::Translate3D: ::new (&translate3_d) (StyleTranslate3D_Body)(other.translate3_d); break;
      case Tag::Scale: ::new (&scale) (StyleScale_Body)(other.scale); break;
      case Tag::ScaleX: ::new (&scale_x) (StyleScaleX_Body)(other.scale_x); break;
      case Tag::ScaleY: ::new (&scale_y) (StyleScaleY_Body)(other.scale_y); break;
      case Tag::ScaleZ: ::new (&scale_z) (StyleScaleZ_Body)(other.scale_z); break;
      case Tag::Scale3D: ::new (&scale3_d) (StyleScale3D_Body)(other.scale3_d); break;
      case Tag::Rotate: ::new (&rotate) (StyleRotate_Body)(other.rotate); break;
      case Tag::RotateX: ::new (&rotate_x) (StyleRotateX_Body)(other.rotate_x); break;
      case Tag::RotateY: ::new (&rotate_y) (StyleRotateY_Body)(other.rotate_y); break;
      case Tag::RotateZ: ::new (&rotate_z) (StyleRotateZ_Body)(other.rotate_z); break;
      case Tag::Rotate3D: ::new (&rotate3_d) (StyleRotate3D_Body)(other.rotate3_d); break;
      case Tag::Perspective: ::new (&perspective) (StylePerspective_Body)(other.perspective); break;
      case Tag::InterpolateMatrix: ::new (&interpolate_matrix) (StyleInterpolateMatrix_Body)(other.interpolate_matrix); break;
      case Tag::AccumulateMatrix: ::new (&accumulate_matrix) (StyleAccumulateMatrix_Body)(other.accumulate_matrix); break;

    }
  }
  StyleGenericTransformOperation& operator=(const StyleGenericTransformOperation& other) {
    if (this != &other) {
      this->~StyleGenericTransformOperation();
      new (this) StyleGenericTransformOperation(other);
    }
    return *this;
  }
};

/// A single operation in a computed CSS `transform`
using StyleTransformOperation = StyleGenericTransformOperation<StyleAngle, StyleNumber, StyleLength, StyleInteger, StyleLengthPercentage>;

/// A computed CSS `transform`
using StyleTransform = StyleGenericTransform<StyleTransformOperation>;

/// The path coord type.
struct StyleCoordPair {
  StyleCSSFloat _0;
  StyleCSSFloat _1;

  bool operator==(const StyleCoordPair& other) const {
    return _0 == other._0 &&
           _1 == other._1;
  }
  bool operator!=(const StyleCoordPair& other) const {
    return _0 != other._0 ||
           _1 != other._1;
  }
  explicit StyleCoordPair(const gfx::Point& aPoint): _0(aPoint.x), _1(aPoint.y) {}
};

/// The EllipticalArc flag type.
struct StyleArcFlag {
  bool _0;

  bool operator==(const StyleArcFlag& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleArcFlag& other) const {
    return _0 != other._0;
  }
};

/// The SVG path command.
/// The fields of these commands are self-explanatory, so we skip the documents.
/// Note: the index of the control points, e.g. control1, control2, are mapping to the control
/// points of the Bézier curve in the spec.
///
/// https://www.w3.org/TR/SVG11/paths.html#PathData
struct StylePathCommand {
  enum class Tag : uint8_t {
    /// The unknown type.
    /// https://www.w3.org/TR/SVG/paths.html#__svg__SVGPathSeg__PATHSEG_UNKNOWN
    Unknown,
    /// The "moveto" command.
    MoveTo,
    /// The "lineto" command.
    LineTo,
    /// The horizontal "lineto" command.
    HorizontalLineTo,
    /// The vertical "lineto" command.
    VerticalLineTo,
    /// The cubic Bézier curve command.
    CurveTo,
    /// The smooth curve command.
    SmoothCurveTo,
    /// The quadratic Bézier curve command.
    QuadBezierCurveTo,
    /// The smooth quadratic Bézier curve command.
    SmoothQuadBezierCurveTo,
    /// The elliptical arc curve command.
    EllipticalArc,
    /// The "closepath" command.
    ClosePath,
  };

  struct StyleMoveTo_Body {
    StyleCoordPair point;
    StyleIsAbsolute absolute;

    bool operator==(const StyleMoveTo_Body& other) const {
      return point == other.point &&
             absolute == other.absolute;
    }
    bool operator!=(const StyleMoveTo_Body& other) const {
      return point != other.point ||
             absolute != other.absolute;
    }
  };

  struct StyleLineTo_Body {
    StyleCoordPair point;
    StyleIsAbsolute absolute;

    bool operator==(const StyleLineTo_Body& other) const {
      return point == other.point &&
             absolute == other.absolute;
    }
    bool operator!=(const StyleLineTo_Body& other) const {
      return point != other.point ||
             absolute != other.absolute;
    }
  };

  struct StyleHorizontalLineTo_Body {
    StyleCSSFloat x;
    StyleIsAbsolute absolute;

    bool operator==(const StyleHorizontalLineTo_Body& other) const {
      return x == other.x &&
             absolute == other.absolute;
    }
    bool operator!=(const StyleHorizontalLineTo_Body& other) const {
      return x != other.x ||
             absolute != other.absolute;
    }
  };

  struct StyleVerticalLineTo_Body {
    StyleCSSFloat y;
    StyleIsAbsolute absolute;

    bool operator==(const StyleVerticalLineTo_Body& other) const {
      return y == other.y &&
             absolute == other.absolute;
    }
    bool operator!=(const StyleVerticalLineTo_Body& other) const {
      return y != other.y ||
             absolute != other.absolute;
    }
  };

  struct StyleCurveTo_Body {
    StyleCoordPair control1;
    StyleCoordPair control2;
    StyleCoordPair point;
    StyleIsAbsolute absolute;

    bool operator==(const StyleCurveTo_Body& other) const {
      return control1 == other.control1 &&
             control2 == other.control2 &&
             point == other.point &&
             absolute == other.absolute;
    }
    bool operator!=(const StyleCurveTo_Body& other) const {
      return control1 != other.control1 ||
             control2 != other.control2 ||
             point != other.point ||
             absolute != other.absolute;
    }
  };

  struct StyleSmoothCurveTo_Body {
    StyleCoordPair control2;
    StyleCoordPair point;
    StyleIsAbsolute absolute;

    bool operator==(const StyleSmoothCurveTo_Body& other) const {
      return control2 == other.control2 &&
             point == other.point &&
             absolute == other.absolute;
    }
    bool operator!=(const StyleSmoothCurveTo_Body& other) const {
      return control2 != other.control2 ||
             point != other.point ||
             absolute != other.absolute;
    }
  };

  struct StyleQuadBezierCurveTo_Body {
    StyleCoordPair control1;
    StyleCoordPair point;
    StyleIsAbsolute absolute;

    bool operator==(const StyleQuadBezierCurveTo_Body& other) const {
      return control1 == other.control1 &&
             point == other.point &&
             absolute == other.absolute;
    }
    bool operator!=(const StyleQuadBezierCurveTo_Body& other) const {
      return control1 != other.control1 ||
             point != other.point ||
             absolute != other.absolute;
    }
  };

  struct StyleSmoothQuadBezierCurveTo_Body {
    StyleCoordPair point;
    StyleIsAbsolute absolute;

    bool operator==(const StyleSmoothQuadBezierCurveTo_Body& other) const {
      return point == other.point &&
             absolute == other.absolute;
    }
    bool operator!=(const StyleSmoothQuadBezierCurveTo_Body& other) const {
      return point != other.point ||
             absolute != other.absolute;
    }
  };

  struct StyleEllipticalArc_Body {
    StyleCSSFloat rx;
    StyleCSSFloat ry;
    StyleCSSFloat angle;
    StyleArcFlag large_arc_flag;
    StyleArcFlag sweep_flag;
    StyleCoordPair point;
    StyleIsAbsolute absolute;

    bool operator==(const StyleEllipticalArc_Body& other) const {
      return rx == other.rx &&
             ry == other.ry &&
             angle == other.angle &&
             large_arc_flag == other.large_arc_flag &&
             sweep_flag == other.sweep_flag &&
             point == other.point &&
             absolute == other.absolute;
    }
    bool operator!=(const StyleEllipticalArc_Body& other) const {
      return rx != other.rx ||
             ry != other.ry ||
             angle != other.angle ||
             large_arc_flag != other.large_arc_flag ||
             sweep_flag != other.sweep_flag ||
             point != other.point ||
             absolute != other.absolute;
    }
  };

  Tag tag;
  union {
    StyleMoveTo_Body move_to;
    StyleLineTo_Body line_to;
    StyleHorizontalLineTo_Body horizontal_line_to;
    StyleVerticalLineTo_Body vertical_line_to;
    StyleCurveTo_Body curve_to;
    StyleSmoothCurveTo_Body smooth_curve_to;
    StyleQuadBezierCurveTo_Body quad_bezier_curve_to;
    StyleSmoothQuadBezierCurveTo_Body smooth_quad_bezier_curve_to;
    StyleEllipticalArc_Body elliptical_arc;
  };

  static StylePathCommand Unknown() {
    StylePathCommand result;
    result.tag = Tag::Unknown;
    return result;
  }

  bool IsUnknown() const {
    return tag == Tag::Unknown;
  }

  static StylePathCommand MoveTo(const StyleCoordPair &point,
                                 const StyleIsAbsolute &absolute) {
    StylePathCommand result;
    ::new (&result.move_to.point) (StyleCoordPair)(point);
    ::new (&result.move_to.absolute) (StyleIsAbsolute)(absolute);
    result.tag = Tag::MoveTo;
    return result;
  }

  bool IsMoveTo() const {
    return tag == Tag::MoveTo;
  }

  const StyleMoveTo_Body& AsMoveTo() const {
    MOZ_DIAGNOSTIC_ASSERT(IsMoveTo());
    return move_to;
  }

  static StylePathCommand LineTo(const StyleCoordPair &point,
                                 const StyleIsAbsolute &absolute) {
    StylePathCommand result;
    ::new (&result.line_to.point) (StyleCoordPair)(point);
    ::new (&result.line_to.absolute) (StyleIsAbsolute)(absolute);
    result.tag = Tag::LineTo;
    return result;
  }

  bool IsLineTo() const {
    return tag == Tag::LineTo;
  }

  const StyleLineTo_Body& AsLineTo() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLineTo());
    return line_to;
  }

  static StylePathCommand HorizontalLineTo(const StyleCSSFloat &x,
                                           const StyleIsAbsolute &absolute) {
    StylePathCommand result;
    ::new (&result.horizontal_line_to.x) (StyleCSSFloat)(x);
    ::new (&result.horizontal_line_to.absolute) (StyleIsAbsolute)(absolute);
    result.tag = Tag::HorizontalLineTo;
    return result;
  }

  bool IsHorizontalLineTo() const {
    return tag == Tag::HorizontalLineTo;
  }

  const StyleHorizontalLineTo_Body& AsHorizontalLineTo() const {
    MOZ_DIAGNOSTIC_ASSERT(IsHorizontalLineTo());
    return horizontal_line_to;
  }

  static StylePathCommand VerticalLineTo(const StyleCSSFloat &y,
                                         const StyleIsAbsolute &absolute) {
    StylePathCommand result;
    ::new (&result.vertical_line_to.y) (StyleCSSFloat)(y);
    ::new (&result.vertical_line_to.absolute) (StyleIsAbsolute)(absolute);
    result.tag = Tag::VerticalLineTo;
    return result;
  }

  bool IsVerticalLineTo() const {
    return tag == Tag::VerticalLineTo;
  }

  const StyleVerticalLineTo_Body& AsVerticalLineTo() const {
    MOZ_DIAGNOSTIC_ASSERT(IsVerticalLineTo());
    return vertical_line_to;
  }

  static StylePathCommand CurveTo(const StyleCoordPair &control1,
                                  const StyleCoordPair &control2,
                                  const StyleCoordPair &point,
                                  const StyleIsAbsolute &absolute) {
    StylePathCommand result;
    ::new (&result.curve_to.control1) (StyleCoordPair)(control1);
    ::new (&result.curve_to.control2) (StyleCoordPair)(control2);
    ::new (&result.curve_to.point) (StyleCoordPair)(point);
    ::new (&result.curve_to.absolute) (StyleIsAbsolute)(absolute);
    result.tag = Tag::CurveTo;
    return result;
  }

  bool IsCurveTo() const {
    return tag == Tag::CurveTo;
  }

  const StyleCurveTo_Body& AsCurveTo() const {
    MOZ_DIAGNOSTIC_ASSERT(IsCurveTo());
    return curve_to;
  }

  static StylePathCommand SmoothCurveTo(const StyleCoordPair &control2,
                                        const StyleCoordPair &point,
                                        const StyleIsAbsolute &absolute) {
    StylePathCommand result;
    ::new (&result.smooth_curve_to.control2) (StyleCoordPair)(control2);
    ::new (&result.smooth_curve_to.point) (StyleCoordPair)(point);
    ::new (&result.smooth_curve_to.absolute) (StyleIsAbsolute)(absolute);
    result.tag = Tag::SmoothCurveTo;
    return result;
  }

  bool IsSmoothCurveTo() const {
    return tag == Tag::SmoothCurveTo;
  }

  const StyleSmoothCurveTo_Body& AsSmoothCurveTo() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSmoothCurveTo());
    return smooth_curve_to;
  }

  static StylePathCommand QuadBezierCurveTo(const StyleCoordPair &control1,
                                            const StyleCoordPair &point,
                                            const StyleIsAbsolute &absolute) {
    StylePathCommand result;
    ::new (&result.quad_bezier_curve_to.control1) (StyleCoordPair)(control1);
    ::new (&result.quad_bezier_curve_to.point) (StyleCoordPair)(point);
    ::new (&result.quad_bezier_curve_to.absolute) (StyleIsAbsolute)(absolute);
    result.tag = Tag::QuadBezierCurveTo;
    return result;
  }

  bool IsQuadBezierCurveTo() const {
    return tag == Tag::QuadBezierCurveTo;
  }

  const StyleQuadBezierCurveTo_Body& AsQuadBezierCurveTo() const {
    MOZ_DIAGNOSTIC_ASSERT(IsQuadBezierCurveTo());
    return quad_bezier_curve_to;
  }

  static StylePathCommand SmoothQuadBezierCurveTo(const StyleCoordPair &point,
                                                  const StyleIsAbsolute &absolute) {
    StylePathCommand result;
    ::new (&result.smooth_quad_bezier_curve_to.point) (StyleCoordPair)(point);
    ::new (&result.smooth_quad_bezier_curve_to.absolute) (StyleIsAbsolute)(absolute);
    result.tag = Tag::SmoothQuadBezierCurveTo;
    return result;
  }

  bool IsSmoothQuadBezierCurveTo() const {
    return tag == Tag::SmoothQuadBezierCurveTo;
  }

  const StyleSmoothQuadBezierCurveTo_Body& AsSmoothQuadBezierCurveTo() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSmoothQuadBezierCurveTo());
    return smooth_quad_bezier_curve_to;
  }

  static StylePathCommand EllipticalArc(const StyleCSSFloat &rx,
                                        const StyleCSSFloat &ry,
                                        const StyleCSSFloat &angle,
                                        const StyleArcFlag &large_arc_flag,
                                        const StyleArcFlag &sweep_flag,
                                        const StyleCoordPair &point,
                                        const StyleIsAbsolute &absolute) {
    StylePathCommand result;
    ::new (&result.elliptical_arc.rx) (StyleCSSFloat)(rx);
    ::new (&result.elliptical_arc.ry) (StyleCSSFloat)(ry);
    ::new (&result.elliptical_arc.angle) (StyleCSSFloat)(angle);
    ::new (&result.elliptical_arc.large_arc_flag) (StyleArcFlag)(large_arc_flag);
    ::new (&result.elliptical_arc.sweep_flag) (StyleArcFlag)(sweep_flag);
    ::new (&result.elliptical_arc.point) (StyleCoordPair)(point);
    ::new (&result.elliptical_arc.absolute) (StyleIsAbsolute)(absolute);
    result.tag = Tag::EllipticalArc;
    return result;
  }

  bool IsEllipticalArc() const {
    return tag == Tag::EllipticalArc;
  }

  const StyleEllipticalArc_Body& AsEllipticalArc() const {
    MOZ_DIAGNOSTIC_ASSERT(IsEllipticalArc());
    return elliptical_arc;
  }

  static StylePathCommand ClosePath() {
    StylePathCommand result;
    result.tag = Tag::ClosePath;
    return result;
  }

  bool IsClosePath() const {
    return tag == Tag::ClosePath;
  }

  bool operator==(const StylePathCommand& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::MoveTo: return move_to == other.move_to;
      case Tag::LineTo: return line_to == other.line_to;
      case Tag::HorizontalLineTo: return horizontal_line_to == other.horizontal_line_to;
      case Tag::VerticalLineTo: return vertical_line_to == other.vertical_line_to;
      case Tag::CurveTo: return curve_to == other.curve_to;
      case Tag::SmoothCurveTo: return smooth_curve_to == other.smooth_curve_to;
      case Tag::QuadBezierCurveTo: return quad_bezier_curve_to == other.quad_bezier_curve_to;
      case Tag::SmoothQuadBezierCurveTo: return smooth_quad_bezier_curve_to == other.smooth_quad_bezier_curve_to;
      case Tag::EllipticalArc: return elliptical_arc == other.elliptical_arc;
      default: break;
    }
    return true;
  }

  bool operator!=(const StylePathCommand& other) const {
    return !(*this == other);
  }

  private:
  StylePathCommand() {

  }
  public:


  ~StylePathCommand() {
    switch (tag) {
      case Tag::MoveTo: move_to.~StyleMoveTo_Body(); break;
      case Tag::LineTo: line_to.~StyleLineTo_Body(); break;
      case Tag::HorizontalLineTo: horizontal_line_to.~StyleHorizontalLineTo_Body(); break;
      case Tag::VerticalLineTo: vertical_line_to.~StyleVerticalLineTo_Body(); break;
      case Tag::CurveTo: curve_to.~StyleCurveTo_Body(); break;
      case Tag::SmoothCurveTo: smooth_curve_to.~StyleSmoothCurveTo_Body(); break;
      case Tag::QuadBezierCurveTo: quad_bezier_curve_to.~StyleQuadBezierCurveTo_Body(); break;
      case Tag::SmoothQuadBezierCurveTo: smooth_quad_bezier_curve_to.~StyleSmoothQuadBezierCurveTo_Body(); break;
      case Tag::EllipticalArc: elliptical_arc.~StyleEllipticalArc_Body(); break;
      default: break;
    }
  }

  StylePathCommand(const StylePathCommand& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::MoveTo: ::new (&move_to) (StyleMoveTo_Body)(other.move_to); break;
      case Tag::LineTo: ::new (&line_to) (StyleLineTo_Body)(other.line_to); break;
      case Tag::HorizontalLineTo: ::new (&horizontal_line_to) (StyleHorizontalLineTo_Body)(other.horizontal_line_to); break;
      case Tag::VerticalLineTo: ::new (&vertical_line_to) (StyleVerticalLineTo_Body)(other.vertical_line_to); break;
      case Tag::CurveTo: ::new (&curve_to) (StyleCurveTo_Body)(other.curve_to); break;
      case Tag::SmoothCurveTo: ::new (&smooth_curve_to) (StyleSmoothCurveTo_Body)(other.smooth_curve_to); break;
      case Tag::QuadBezierCurveTo: ::new (&quad_bezier_curve_to) (StyleQuadBezierCurveTo_Body)(other.quad_bezier_curve_to); break;
      case Tag::SmoothQuadBezierCurveTo: ::new (&smooth_quad_bezier_curve_to) (StyleSmoothQuadBezierCurveTo_Body)(other.smooth_quad_bezier_curve_to); break;
      case Tag::EllipticalArc: ::new (&elliptical_arc) (StyleEllipticalArc_Body)(other.elliptical_arc); break;
      default: break;
    }
  }
  StylePathCommand& operator=(const StylePathCommand& other) {
    if (this != &other) {
      this->~StylePathCommand();
      new (this) StylePathCommand(other);
    }
    return *this;
  }
};

/// Header data with an inline length. Consumers that use HeaderWithLength as the
/// Header type in HeaderSlice can take advantage of ThinArc.
template<typename H>
struct StyleHeaderWithLength {
  /// The fixed-sized data.
  H header;
  /// The slice length.
  uintptr_t length;

  bool operator==(const StyleHeaderWithLength& other) const {
    return header == other.header &&
           length == other.length;
  }
  bool operator!=(const StyleHeaderWithLength& other) const {
    return header != other.header ||
           length != other.length;
  }
};

/// Structure to allow Arc-managing some fixed-sized data and a variably-sized
/// slice in a single allocation.
template<typename H, typename T>
struct StyleHeaderSlice {
  /// The fixed-sized data.
  H header;
  /// The dynamically-sized data.
  T slice;

  bool operator==(const StyleHeaderSlice& other) const {
    return header == other.header &&
           slice == other.slice;
  }
  bool operator!=(const StyleHeaderSlice& other) const {
    return header != other.header ||
           slice != other.slice;
  }
};

template<typename H, typename T>
using StyleHeaderSliceWithLength = StyleHeaderSlice<StyleHeaderWithLength<H>, T>;

/// The object allocated by an Arc<T>
template<typename T>
struct StyleArcInner {
  StyleAtomicUsize count;
  T data;

  bool operator==(const StyleArcInner& other) const {
    return count == other.count &&
           data == other.data;
  }
  bool operator!=(const StyleArcInner& other) const {
    return count != other.count ||
           data != other.data;
  }
  // Increase the reference count.
  inline void IncrementRef();
  // Release the reference count, and return whether the result must be freed or not.
  [[nodiscard]] inline bool DecrementRef();
};

/// A "thin" `Arc` containing dynamically sized data
///
/// This is functionally equivalent to Arc<(H, [T])>
///
/// When you create an `Arc` containing a dynamically sized type
/// like `HeaderSlice<H, [T]>`, the `Arc` is represented on the stack
/// as a "fat pointer", where the length of the slice is stored
/// alongside the `Arc`'s pointer. In some situations you may wish to
/// have a thin pointer instead, perhaps for FFI compatibility
/// or space efficiency.
///
/// Note that we use `[T; 0]` in order to have the right alignment for `T`.
///
/// `ThinArc` solves this by storing the length in the allocation itself,
/// via `HeaderSliceWithLength`.
template<typename H, typename T>
struct StyleThinArc {
  StyleArcInner<StyleHeaderSliceWithLength<H, T[0]>> *ptr;

  bool operator==(const StyleThinArc& other) const {
    return ptr == other.ptr;
  }
  bool operator!=(const StyleThinArc& other) const {
    return ptr != other.ptr;
  }
};

/// A wrapper type for a refcounted slice using ThinArc.
///
template<typename T>
struct StyleArcSlice {
  StyleThinArc<uint64_t, T> _0;
  inline StyleArcSlice();
  inline StyleArcSlice(const StyleArcSlice& aOther);

  // Should be easily implementable if wanted, but the default implementation would leak.
  StyleArcSlice& operator=(const StyleArcSlice&) = delete;
  StyleArcSlice& operator=(StyleArcSlice&&) = delete;

  inline explicit StyleArcSlice(const StyleForgottenArcSlicePtr<T>& aPtr);
  inline ~StyleArcSlice();
  inline Span<const T> AsSpan() const;
  inline size_t Length() const;
  inline bool IsEmpty() const;
  inline bool operator==(const StyleArcSlice& other) const;
  inline bool operator!=(const StyleArcSlice& other) const;
};

/// The SVG path data.
///
/// https://www.w3.org/TR/SVG11/paths.html#PathData
struct StyleSVGPathData {
  StyleArcSlice<StylePathCommand> _0;

  bool operator==(const StyleSVGPathData& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleSVGPathData& other) const {
    return _0 != other._0;
  }
};

/// The `ray()` function, `ray( [ <angle> && <size> && contain? ] )`
///
/// https://drafts.fxtf.org/motion-1/#valdef-offsetpath-ray
template<typename Angle>
struct StyleRayFunction {
  /// The bearing angle with `0deg` pointing up and positive angles
  /// representing clockwise rotation.
  Angle angle;
  /// Decide the path length used when `offset-distance` is expressed
  /// as a percentage.
  StyleRaySize size;
  /// Clamp `offset-distance` so that the box is entirely contained
  /// within the path.
  bool contain;

  bool operator==(const StyleRayFunction& other) const {
    return angle == other.angle &&
           size == other.size &&
           contain == other.contain;
  }
  bool operator!=(const StyleRayFunction& other) const {
    return angle != other.angle ||
           size != other.size ||
           contain != other.contain;
  }
};

/// The offset-path value.
///
/// https://drafts.fxtf.org/motion-1/#offset-path-property
template<typename Angle>
struct StyleGenericOffsetPath {
  enum class Tag : uint8_t {
    /// Path value for path(<string>).
    Path,
    /// ray() function, which defines a path in the polar coordinate system.
    Ray,
    /// None value.
    None,
  };

  struct StylePath_Body {
    StyleSVGPathData _0;

    bool operator==(const StylePath_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StylePath_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleRay_Body {
    StyleRayFunction<Angle> _0;

    bool operator==(const StyleRay_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleRay_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StylePath_Body path;
    StyleRay_Body ray;
  };

  static StyleGenericOffsetPath Path(const StyleSVGPathData &_0) {
    StyleGenericOffsetPath result;
    ::new (&result.path._0) (StyleSVGPathData)(_0);
    result.tag = Tag::Path;
    return result;
  }

  bool IsPath() const {
    return tag == Tag::Path;
  }

  const StyleSVGPathData& AsPath() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPath());
    return path._0;
  }

  static StyleGenericOffsetPath Ray(const StyleRayFunction<Angle> &_0) {
    StyleGenericOffsetPath result;
    ::new (&result.ray._0) (StyleRayFunction<Angle>)(_0);
    result.tag = Tag::Ray;
    return result;
  }

  bool IsRay() const {
    return tag == Tag::Ray;
  }

  const StyleRayFunction<Angle>& AsRay() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRay());
    return ray._0;
  }

  static StyleGenericOffsetPath None() {
    StyleGenericOffsetPath result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  bool operator==(const StyleGenericOffsetPath& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Path: return path == other.path;
      case Tag::Ray: return ray == other.ray;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericOffsetPath& other) const {
    return !(*this == other);
  }

  ~StyleGenericOffsetPath() {
    switch (tag) {
      case Tag::Path: path.~StylePath_Body(); break;
      case Tag::Ray: ray.~StyleRay_Body(); break;
      default: break;
    }
  }

  StyleGenericOffsetPath(const StyleGenericOffsetPath& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Path: ::new (&path) (StylePath_Body)(other.path); break;
      case Tag::Ray: ::new (&ray) (StyleRay_Body)(other.ray); break;
      default: break;
    }
  }
  StyleGenericOffsetPath& operator=(const StyleGenericOffsetPath& other) {
    if (this != &other) {
      this->~StyleGenericOffsetPath();
      new (this) StyleGenericOffsetPath(other);
    }
    return *this;
  }
 public:
  // The implementation of IPC LayersMessages needs this to be public.
  StyleGenericOffsetPath(): tag(Tag::None) {}
};

/// The computed value of `offset-path`.
using StyleOffsetPath = StyleGenericOffsetPath<StyleAngle>;

/// A computed offset-rotate.
struct StyleOffsetRotate {
  /// If auto is false, this is a fixed angle which indicates a
  /// constant clockwise rotation transformation applied to it by this
  /// specified rotation angle. Otherwise, the angle will be added to
  /// the angle of the direction in layout.
  bool auto_;
  /// The angle value.
  StyleAngle angle;

  bool operator==(const StyleOffsetRotate& other) const {
    return auto_ == other.auto_ &&
           angle == other.angle;
  }
  bool operator!=(const StyleOffsetRotate& other) const {
    return auto_ != other.auto_ ||
           angle != other.angle;
  }
};

/// The computed value of a CSS horizontal position.
using StyleHorizontalPosition = StyleLengthPercentage;

/// The computed value of a CSS vertical position.
using StyleVerticalPosition = StyleLengthPercentage;

/// A generic type for representing a CSS [position](https://drafts.csswg.org/css-values/#position).
template<typename H, typename V>
struct StyleGenericPosition {
  /// The horizontal component of position.
  H horizontal;
  /// The vertical component of position.
  V vertical;

  bool operator==(const StyleGenericPosition& other) const {
    return horizontal == other.horizontal &&
           vertical == other.vertical;
  }
  bool operator!=(const StyleGenericPosition& other) const {
    return horizontal != other.horizontal ||
           vertical != other.vertical;
  }
  inline bool HasPercent() const;
  inline bool DependsOnPositioningAreaSize() const;
  static inline StyleGenericPosition FromPercentage(float);
};

/// The computed value of a CSS `<position>`
using StylePosition = StyleGenericPosition<StyleHorizontalPosition, StyleVerticalPosition>;

/// A generic type for representing an `Auto | <position>`.
/// This is used by <offset-anchor> for now.
/// https://drafts.fxtf.org/motion-1/#offset-anchor-property
template<typename Pos>
struct StyleGenericPositionOrAuto {
  enum class Tag : uint8_t {
    /// The <position> value.
    Position,
    /// The keyword `auto`.
    Auto,
  };

  struct StylePosition_Body {
    Pos _0;

    bool operator==(const StylePosition_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StylePosition_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StylePosition_Body position;
  };

  static StyleGenericPositionOrAuto Position(const Pos &_0) {
    StyleGenericPositionOrAuto result;
    ::new (&result.position._0) (Pos)(_0);
    result.tag = Tag::Position;
    return result;
  }

  bool IsPosition() const {
    return tag == Tag::Position;
  }

  const Pos& AsPosition() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPosition());
    return position._0;
  }

  static StyleGenericPositionOrAuto Auto() {
    StyleGenericPositionOrAuto result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  bool operator==(const StyleGenericPositionOrAuto& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Position: return position == other.position;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericPositionOrAuto& other) const {
    return !(*this == other);
  }

  ~StyleGenericPositionOrAuto() {
    switch (tag) {
      case Tag::Position: position.~StylePosition_Body(); break;
      default: break;
    }
  }

  StyleGenericPositionOrAuto(const StyleGenericPositionOrAuto& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Position: ::new (&position) (StylePosition_Body)(other.position); break;
      default: break;
    }
  }
  StyleGenericPositionOrAuto& operator=(const StyleGenericPositionOrAuto& other) {
    if (this != &other) {
      this->~StyleGenericPositionOrAuto();
      new (this) StyleGenericPositionOrAuto(other);
    }
    return *this;
  }
 public:
  // The implementation of IPC LayersMessages needs this to be public.
  StyleGenericPositionOrAuto(): tag(Tag::Auto) {}
};

/// The computed value of an `auto | <position>`
using StylePositionOrAuto = StyleGenericPositionOrAuto<StylePosition>;

#if defined(CBINDGEN_IS_GECKO)
/// Gecko-FFI-safe owned pointer.
///
/// Cannot be null, and leaks on drop, so needs to be converted into a rust-side
/// `Box` before.
template<typename GeckoType>
struct StyleOwned {
  GeckoType *ptr;

  bool operator==(const StyleOwned& other) const {
    return ptr == other.ptr;
  }
  bool operator!=(const StyleOwned& other) const {
    return ptr != other.ptr;
  }
  UniquePtr<GeckoType> Consume() {
    UniquePtr<GeckoType> ret(ptr);
    ptr = nullptr;
    return ret;
  }
};
#endif

/// The computed representation of the above so Gecko can read them easily.
///
/// This one is needed because cbindgen doesn't know how to generate
/// specified::Number.
struct StyleComputedFontWeightRange {
  float _0;
  float _1;

  bool operator==(const StyleComputedFontWeightRange& other) const {
    return _0 == other._0 &&
           _1 == other._1;
  }
  bool operator!=(const StyleComputedFontWeightRange& other) const {
    return _0 != other._0 ||
           _1 != other._1;
  }
};

/// The computed representation of the above, so that
/// Gecko can read them easily.
struct StyleComputedFontStretchRange {
  float _0;
  float _1;

  bool operator==(const StyleComputedFontStretchRange& other) const {
    return _0 == other._0 &&
           _1 == other._1;
  }
  bool operator!=(const StyleComputedFontStretchRange& other) const {
    return _0 != other._0 ||
           _1 != other._1;
  }
};

/// The computed representation of the above, with angles in degrees, so that
/// Gecko can read them easily.
union StyleComputedFontStyleDescriptor {
  enum class Tag : uint8_t {
    Normal,
    Italic,
    Oblique,
  };

  struct Oblique_Body {
    Tag tag;
    float _0;
    float _1;

    bool operator==(const Oblique_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const Oblique_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct {
    Tag tag;
  };
  Oblique_Body oblique;

  static StyleComputedFontStyleDescriptor Normal() {
    StyleComputedFontStyleDescriptor result;
    result.tag = Tag::Normal;
    return result;
  }

  bool IsNormal() const {
    return tag == Tag::Normal;
  }

  static StyleComputedFontStyleDescriptor Italic() {
    StyleComputedFontStyleDescriptor result;
    result.tag = Tag::Italic;
    return result;
  }

  bool IsItalic() const {
    return tag == Tag::Italic;
  }

  static StyleComputedFontStyleDescriptor Oblique(const float &_0,
                                                  const float &_1) {
    StyleComputedFontStyleDescriptor result;
    ::new (&result.oblique._0) (float)(_0);
    ::new (&result.oblique._1) (float)(_1);
    result.tag = Tag::Oblique;
    return result;
  }

  bool IsOblique() const {
    return tag == Tag::Oblique;
  }

  const Oblique_Body& AsOblique() const {
    MOZ_DIAGNOSTIC_ASSERT(IsOblique());
    return oblique;
  }

  bool operator==(const StyleComputedFontStyleDescriptor& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Oblique: return oblique == other.oblique;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleComputedFontStyleDescriptor& other) const {
    return !(*this == other);
  }

  private:
  StyleComputedFontStyleDescriptor() {

  }
  public:


  ~StyleComputedFontStyleDescriptor() {
    switch (tag) {
      case Tag::Oblique: oblique.~Oblique_Body(); break;
      default: break;
    }
  }

  StyleComputedFontStyleDescriptor(const StyleComputedFontStyleDescriptor& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Oblique: ::new (&oblique) (Oblique_Body)(other.oblique); break;
      default: break;
    }
  }
  StyleComputedFontStyleDescriptor& operator=(const StyleComputedFontStyleDescriptor& other) {
    if (this != &other) {
      this->~StyleComputedFontStyleDescriptor();
      new (this) StyleComputedFontStyleDescriptor(other);
    }
    return *this;
  }
};

/// font-language-override can only have a single three-letter
/// OpenType "language system" tag, so we should be able to compute
/// it and store it as a 32-bit integer
/// (see http://www.microsoft.com/typography/otspec/languagetags.htm).
struct StyleFontLanguageOverride {
  uint32_t _0;

  bool operator==(const StyleFontLanguageOverride& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleFontLanguageOverride& other) const {
    return _0 != other._0;
  }
};

/// One contiguous range of code points.
///
/// Can not be empty. Can represent a single code point when start == end.
struct StyleUnicodeRange {
  /// Inclusive start of the range. In [0, end].
  uint32_t start;
  /// Inclusive end of the range. In [0, 0x10FFFF].
  uint32_t end;

  bool operator==(const StyleUnicodeRange& other) const {
    return start == other.start &&
           end == other.end;
  }
  bool operator!=(const StyleUnicodeRange& other) const {
    return start != other.start ||
           end != other.end;
  }
};

/// A struct that basically replaces a Box<str>, but with a defined layout,
/// suitable for FFI.
struct StyleOwnedStr {
  StyleOwnedSlice<uint8_t> _0;

  bool operator==(const StyleOwnedStr& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleOwnedStr& other) const {
    return _0 != other._0;
  }
  inline nsDependentCSubstring AsString() const;
};

#if defined(CBINDGEN_IS_GECKO)
/// Extra data that the backend may need to resolve url values.
///
/// If the usize's lowest bit is 0, then this is a strong reference to a
/// structs::URLExtraData object.
///
/// Otherwise, shifting the usize's bits the right by one gives the
/// UserAgentStyleSheetID value corresponding to the style sheet whose
/// URLExtraData this is, which is stored in URLExtraData_sShared.  We don't
/// hold a strong reference to that object from here, but we rely on that
/// array's objects being held alive until shutdown.
///
/// We use this packed representation rather than an enum so that
/// `from_ptr_ref` can work.
struct StyleUrlExtraData {
  uintptr_t _0;

  bool operator==(const StyleUrlExtraData& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleUrlExtraData& other) const {
    return _0 != other._0;
  }
  StyleUrlExtraData() = delete;

  // Could be implemented if wanted.
  StyleUrlExtraData(const StyleUrlExtraData&) = delete;
  StyleUrlExtraData& operator=(const StyleUrlExtraData&) = delete;

  inline bool IsShared() const;

  inline ~StyleUrlExtraData();
  inline const URLExtraData& get() const;
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// Various bits of mutable state that are kept for image loads.
struct StyleLoadDataFlags {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleLoadDataFlags operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleLoadDataFlags operator|(const StyleLoadDataFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleLoadDataFlags& operator|=(const StyleLoadDataFlags& other) {
    *this = (*this | other);
    return *this;
  }
  StyleLoadDataFlags operator&(const StyleLoadDataFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleLoadDataFlags& operator&=(const StyleLoadDataFlags& other) {
    *this = (*this & other);
    return *this;
  }
  StyleLoadDataFlags operator^(const StyleLoadDataFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleLoadDataFlags& operator^=(const StyleLoadDataFlags& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleLoadDataFlags& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleLoadDataFlags& other) const {
    return bits != other.bits;
  }
  static const StyleLoadDataFlags TRIED_TO_RESOLVE_URI;
  static const StyleLoadDataFlags TRIED_TO_RESOLVE_IMAGE;
};
#if defined(CBINDGEN_IS_GECKO)
/// Whether we tried to resolve the uri at least once.
inline const StyleLoadDataFlags StyleLoadDataFlags::TRIED_TO_RESOLVE_URI = StyleLoadDataFlags{ /* .bits = */ (uint8_t)(1 << 0) };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// Whether we tried to resolve the image at least once.
inline const StyleLoadDataFlags StyleLoadDataFlags::TRIED_TO_RESOLVE_IMAGE = StyleLoadDataFlags{ /* .bits = */ (uint8_t)(1 << 1) };
#endif
#endif

#if defined(CBINDGEN_IS_GECKO)
/// The load data for a given URL. This is mutable from C++, and shouldn't be
/// accessed from rust for anything.
struct StyleLoadData {
  /// A strong reference to the imgRequestProxy, if any, that should be
  /// released on drop.
  ///
  /// These are raw pointers because they are not safe to reference-count off
  /// the main thread.
  imgRequestProxy *resolved_image;
  /// A strong reference to the resolved URI of this image.
  nsIURI *resolved_uri;
  /// A few flags that are set when resolving the image or such.
  StyleLoadDataFlags flags;

  bool operator==(const StyleLoadData& other) const {
    return resolved_image == other.resolved_image &&
           resolved_uri == other.resolved_uri &&
           flags == other.flags;
  }
  bool operator!=(const StyleLoadData& other) const {
    return resolved_image != other.resolved_image ||
           resolved_uri != other.resolved_uri ||
           flags != other.flags;
  }
  ~StyleLoadData();
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// The data for a load, or a lazy-loaded, static member that will be stored in
/// LOAD_DATA_TABLE, keyed by the memory location of this object, which is
/// always in the heap because it's inside the CssUrlData object.
///
/// This type is meant not to be used from C++ so we don't derive helper
/// methods.
///
struct StyleLoadDataSource {
  enum class Tag : uint8_t {
#if defined(CBINDGEN_IS_GECKO)
    /// An owned copy of the load data.
    Owned,
#endif
#if defined(CBINDGEN_IS_GECKO)
    /// A lazily-resolved copy of it.
    Lazy,
#endif
  };

#if defined(CBINDGEN_IS_GECKO)
  struct StyleOwned_Body {
    StyleLoadData _0;

    bool operator==(const StyleOwned_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleOwned_Body& other) const {
      return _0 != other._0;
    }
  };
#endif

  Tag tag;
  union {
#if defined(CBINDGEN_IS_GECKO)
    StyleOwned_Body owned;
#endif
  };

  bool operator==(const StyleLoadDataSource& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
#if defined(CBINDGEN_IS_GECKO)
      case Tag::Owned: return owned == other.owned;
#endif
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleLoadDataSource& other) const {
    return !(*this == other);
  }

  private:
  StyleLoadDataSource() {

  }
  public:


  ~StyleLoadDataSource() {
    switch (tag) {
#if defined(CBINDGEN_IS_GECKO)
      case Tag::Owned: owned.~StyleOwned_Body(); break;
#endif
      default: break;
    }
  }

  StyleLoadDataSource(const StyleLoadDataSource& other)
   : tag(other.tag) {
    switch (tag) {
#if defined(CBINDGEN_IS_GECKO)
      case Tag::Owned: ::new (&owned) (StyleOwned_Body)(other.owned); break;
#endif
      default: break;
    }
  }
  StyleLoadDataSource& operator=(const StyleLoadDataSource& other) {
    if (this != &other) {
      this->~StyleLoadDataSource();
      new (this) StyleLoadDataSource(other);
    }
    return *this;
  }
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// Data shared between CssUrls.
///
struct StyleCssUrlData {
  /// The URL in unresolved string form.
  StyleOwnedStr serialization;
  /// The URL extra data.
  StyleUrlExtraData extra_data;
  /// The CORS mode that will be used for the load.
  StyleCorsMode cors_mode;
  /// Data to trigger a load from Gecko. This is mutable in C++.
  ///
  /// TODO(emilio): Maybe we can eagerly resolve URLs and make this immutable?
  StyleLoadDataSource load_data;
  // Implemented in nsStyleStruct.cpp
  bool operator==(const StyleCssUrlData& other) const;
  bool operator!=(const StyleCssUrlData& other) const {
    return !(*this == other);
  }
};
#endif

/// An atomically reference counted shared pointer
///
/// See the documentation for [`Arc`] in the standard library. Unlike the
/// standard library `Arc`, this `Arc` does not support weak reference counting.
///
/// See the discussion in https://github.com/rust-lang/rust/pull/60594 for the
/// usage of PhantomData.
///
/// [`Arc`]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
///
template<typename T>
struct StyleArc {
  StyleArcInner<T> *p;
  StyleArc() = delete;
  inline StyleArc(const StyleArc& Other);
 private:
  inline void Release();
 public:
  inline ~StyleArc();

  inline StyleArc& operator=(const StyleArc&);
  inline StyleArc& operator=(StyleArc&&);

  const T* operator->() const {
    MOZ_DIAGNOSTIC_ASSERT(p, "Arc shouldn't be null");
    return &p->data;
  }
  const T& operator*() const {
    MOZ_DIAGNOSTIC_ASSERT(p, "Arc shouldn't be null");
    return p->data;
  }
  bool operator==(const StyleArc& other) const {
    return p == other.p || *(*this) == *other;
  }
  bool operator!=(const StyleArc& other) const {
    return !(*this == other);
  }
};

#if defined(CBINDGEN_IS_GECKO)
/// A CSS url() value for gecko.
struct StyleCssUrl {
  StyleArc<StyleCssUrlData> _0;

  bool operator==(const StyleCssUrl& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleCssUrl& other) const {
    return _0 != other._0;
  }
  inline nsDependentCSubstring SpecifiedSerialization() const;
  inline const URLExtraData& ExtraData() const;
  inline StyleLoadData& LoadData() const;
  inline nsIURI* GetURI() const;
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// A POD representation for Gecko. All pointers here are non-owned and as such
/// can't outlive the rule they came from, but we can't enforce that via C++.
///
/// All the strings are of course utf8.
union StyleFontFaceSourceListComponent {
  enum class Tag : uint8_t {
    Url,
    Local,
    FormatHint,
  };

  struct Url_Body {
    Tag tag;
    const StyleCssUrl *_0;

    bool operator==(const Url_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Url_Body& other) const {
      return _0 != other._0;
    }
  };

  struct Local_Body {
    Tag tag;
    nsAtom *_0;

    bool operator==(const Local_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Local_Body& other) const {
      return _0 != other._0;
    }
  };

  struct FormatHint_Body {
    Tag tag;
    uintptr_t length;
    const uint8_t *utf8_bytes;

    bool operator==(const FormatHint_Body& other) const {
      return length == other.length &&
             utf8_bytes == other.utf8_bytes;
    }
    bool operator!=(const FormatHint_Body& other) const {
      return length != other.length ||
             utf8_bytes != other.utf8_bytes;
    }
  };

  struct {
    Tag tag;
  };
  Url_Body url;
  Local_Body local;
  FormatHint_Body format_hint;

  static StyleFontFaceSourceListComponent Url(const StyleCssUrl *const &_0) {
    StyleFontFaceSourceListComponent result;
    ::new (&result.url._0) (const StyleCssUrl*)(_0);
    result.tag = Tag::Url;
    return result;
  }

  bool IsUrl() const {
    return tag == Tag::Url;
  }

  const StyleCssUrl*const & AsUrl() const {
    MOZ_DIAGNOSTIC_ASSERT(IsUrl());
    return url._0;
  }

  static StyleFontFaceSourceListComponent Local(nsAtom *const &_0) {
    StyleFontFaceSourceListComponent result;
    ::new (&result.local._0) (nsAtom*)(_0);
    result.tag = Tag::Local;
    return result;
  }

  bool IsLocal() const {
    return tag == Tag::Local;
  }

  nsAtom*const & AsLocal() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLocal());
    return local._0;
  }

  static StyleFontFaceSourceListComponent FormatHint(const uintptr_t &length,
                                                     const uint8_t *const &utf8_bytes) {
    StyleFontFaceSourceListComponent result;
    ::new (&result.format_hint.length) (uintptr_t)(length);
    ::new (&result.format_hint.utf8_bytes) (const uint8_t*)(utf8_bytes);
    result.tag = Tag::FormatHint;
    return result;
  }

  bool IsFormatHint() const {
    return tag == Tag::FormatHint;
  }

  const FormatHint_Body& AsFormatHint() const {
    MOZ_DIAGNOSTIC_ASSERT(IsFormatHint());
    return format_hint;
  }

  bool operator==(const StyleFontFaceSourceListComponent& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Url: return url == other.url;
      case Tag::Local: return local == other.local;
      case Tag::FormatHint: return format_hint == other.format_hint;

    }
    return true;
  }

  bool operator!=(const StyleFontFaceSourceListComponent& other) const {
    return !(*this == other);
  }

  private:
  StyleFontFaceSourceListComponent() {

  }
  public:


  ~StyleFontFaceSourceListComponent() {
    switch (tag) {
      case Tag::Url: url.~Url_Body(); break;
      case Tag::Local: local.~Local_Body(); break;
      case Tag::FormatHint: format_hint.~FormatHint_Body(); break;

    }
  }

  StyleFontFaceSourceListComponent(const StyleFontFaceSourceListComponent& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Url: ::new (&url) (Url_Body)(other.url); break;
      case Tag::Local: ::new (&local) (Local_Body)(other.local); break;
      case Tag::FormatHint: ::new (&format_hint) (FormatHint_Body)(other.format_hint); break;

    }
  }
  StyleFontFaceSourceListComponent& operator=(const StyleFontFaceSourceListComponent& other) {
    if (this != &other) {
      this->~StyleFontFaceSourceListComponent();
      new (this) StyleFontFaceSourceListComponent(other);
    }
    return *this;
  }
};
#endif

struct StyleAdditiveSymbol {
  int32_t weight;
  nsString symbol;

  bool operator==(const StyleAdditiveSymbol& other) const {
    return weight == other.weight &&
           symbol == other.symbol;
  }
  bool operator!=(const StyleAdditiveSymbol& other) const {
    return weight != other.weight ||
           symbol != other.symbol;
  }
};

struct StyleCounterSpeakAs {
  enum class Tag : uint8_t {
    None,
    Auto,
    Bullets,
    Numbers,
    Words,
    Ident,
  };

  struct StyleIdent_Body {
    nsAtom *_0;

    bool operator==(const StyleIdent_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleIdent_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleIdent_Body ident;
  };

  static StyleCounterSpeakAs None() {
    StyleCounterSpeakAs result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleCounterSpeakAs Auto() {
    StyleCounterSpeakAs result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  static StyleCounterSpeakAs Bullets() {
    StyleCounterSpeakAs result;
    result.tag = Tag::Bullets;
    return result;
  }

  bool IsBullets() const {
    return tag == Tag::Bullets;
  }

  static StyleCounterSpeakAs Numbers() {
    StyleCounterSpeakAs result;
    result.tag = Tag::Numbers;
    return result;
  }

  bool IsNumbers() const {
    return tag == Tag::Numbers;
  }

  static StyleCounterSpeakAs Words() {
    StyleCounterSpeakAs result;
    result.tag = Tag::Words;
    return result;
  }

  bool IsWords() const {
    return tag == Tag::Words;
  }

  static StyleCounterSpeakAs Ident(nsAtom *const &_0) {
    StyleCounterSpeakAs result;
    ::new (&result.ident._0) (nsAtom*)(_0);
    result.tag = Tag::Ident;
    return result;
  }

  bool IsIdent() const {
    return tag == Tag::Ident;
  }

  nsAtom*const & AsIdent() const {
    MOZ_DIAGNOSTIC_ASSERT(IsIdent());
    return ident._0;
  }

  bool operator==(const StyleCounterSpeakAs& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Ident: return ident == other.ident;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleCounterSpeakAs& other) const {
    return !(*this == other);
  }

  private:
  StyleCounterSpeakAs() {

  }
  public:


  ~StyleCounterSpeakAs() {
    switch (tag) {
      case Tag::Ident: ident.~StyleIdent_Body(); break;
      default: break;
    }
  }

  StyleCounterSpeakAs(const StyleCounterSpeakAs& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Ident: ::new (&ident) (StyleIdent_Body)(other.ident); break;
      default: break;
    }
  }
  StyleCounterSpeakAs& operator=(const StyleCounterSpeakAs& other) {
    if (this != &other) {
      this->~StyleCounterSpeakAs();
      new (this) StyleCounterSpeakAs(other);
    }
    return *this;
  }
};

/// The kind of restyle we need to do for a given element.
struct StyleRestyleHint {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleRestyleHint operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleRestyleHint operator|(const StyleRestyleHint& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleRestyleHint& operator|=(const StyleRestyleHint& other) {
    *this = (*this | other);
    return *this;
  }
  StyleRestyleHint operator&(const StyleRestyleHint& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleRestyleHint& operator&=(const StyleRestyleHint& other) {
    *this = (*this & other);
    return *this;
  }
  StyleRestyleHint operator^(const StyleRestyleHint& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleRestyleHint& operator^=(const StyleRestyleHint& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleRestyleHint& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleRestyleHint& other) const {
    return bits != other.bits;
  }
  static inline StyleRestyleHint RestyleSubtree();
  static inline StyleRestyleHint RecascadeSubtree();
  static inline StyleRestyleHint ForAnimations();
  // Returns true if this change hint is guaranteed to at least recascade all
  // elements in the subtree of the element it is applied to.
  inline bool DefinitelyRecascadesAllSubtree() const;
  static const StyleRestyleHint RESTYLE_SELF;
  static const StyleRestyleHint RESTYLE_DESCENDANTS;
  static const StyleRestyleHint RECASCADE_SELF;
  static const StyleRestyleHint RECASCADE_DESCENDANTS;
  static const StyleRestyleHint RESTYLE_CSS_TRANSITIONS;
  static const StyleRestyleHint RESTYLE_CSS_ANIMATIONS;
  static const StyleRestyleHint RESTYLE_STYLE_ATTRIBUTE;
  static const StyleRestyleHint RESTYLE_SMIL;
};
/// Do a selector match of the element.
inline const StyleRestyleHint StyleRestyleHint::RESTYLE_SELF = StyleRestyleHint{ /* .bits = */ (uint8_t)(1 << 0) };
/// Do a selector match of the element's descendants.
inline const StyleRestyleHint StyleRestyleHint::RESTYLE_DESCENDANTS = StyleRestyleHint{ /* .bits = */ (uint8_t)(1 << 1) };
/// Recascade the current element.
inline const StyleRestyleHint StyleRestyleHint::RECASCADE_SELF = StyleRestyleHint{ /* .bits = */ (uint8_t)(1 << 2) };
/// Recascade all descendant elements.
inline const StyleRestyleHint StyleRestyleHint::RECASCADE_DESCENDANTS = StyleRestyleHint{ /* .bits = */ (uint8_t)(1 << 3) };
/// Replace the style data coming from CSS transitions without updating
/// any other style data. This hint is only processed in animation-only
/// traversal which is prior to normal traversal.
inline const StyleRestyleHint StyleRestyleHint::RESTYLE_CSS_TRANSITIONS = StyleRestyleHint{ /* .bits = */ (uint8_t)(1 << 4) };
/// Replace the style data coming from CSS animations without updating
/// any other style data. This hint is only processed in animation-only
/// traversal which is prior to normal traversal.
inline const StyleRestyleHint StyleRestyleHint::RESTYLE_CSS_ANIMATIONS = StyleRestyleHint{ /* .bits = */ (uint8_t)(1 << 5) };
/// Don't re-run selector-matching on the element, only the style
/// attribute has changed, and this change didn't have any other
/// dependencies.
inline const StyleRestyleHint StyleRestyleHint::RESTYLE_STYLE_ATTRIBUTE = StyleRestyleHint{ /* .bits = */ (uint8_t)(1 << 6) };
/// Replace the style data coming from SMIL animations without updating
/// any other style data. This hint is only processed in animation-only
/// traversal which is prior to normal traversal.
inline const StyleRestyleHint StyleRestyleHint::RESTYLE_SMIL = StyleRestyleHint{ /* .bits = */ (uint8_t)(1 << 7) };

#if defined(CBINDGEN_IS_GECKO)
/// Gecko-FFI-safe owned pointer.
///
/// Can be null, and just as `Owned` leaks on `Drop`.
template<typename GeckoType>
struct StyleOwnedOrNull {
  GeckoType *ptr;

  bool operator==(const StyleOwnedOrNull& other) const {
    return ptr == other.ptr;
  }
  bool operator!=(const StyleOwnedOrNull& other) const {
    return ptr != other.ptr;
  }
  UniquePtr<GeckoType> Consume() {
    UniquePtr<GeckoType> ret(ptr);
    ptr = nullptr;
    return ret;
  }
};
#endif

/// A CSS value made of four components, where its `ToCss` impl will try to
/// serialize as few components as possible, like for example in `border-width`.
template<typename T>
struct StyleRect {
  T _0;
  T _1;
  T _2;
  T _3;

  bool operator==(const StyleRect& other) const {
    return _0 == other._0 &&
           _1 == other._1 &&
           _2 == other._2 &&
           _3 == other._3;
  }
  bool operator!=(const StyleRect& other) const {
    return _0 != other._0 ||
           _1 != other._1 ||
           _2 != other._2 ||
           _3 != other._3;
  }
  template<typename Predicate> inline bool All(Predicate) const;
  template<typename Predicate> inline bool Any(Predicate) const;

  // Defined in WritingModes.h
  inline const T& Get(mozilla::Side) const;
  inline const T& Get(WritingMode, LogicalSide) const;
  inline const T& GetIStart(WritingMode) const;
  inline const T& GetBStart(WritingMode) const;
  inline const T& Start(LogicalAxis, WritingMode) const;
  inline const T& GetIEnd(WritingMode) const;
  inline const T& GetBEnd(WritingMode) const;
  inline const T& End(LogicalAxis, WritingMode) const;

  inline T& Get(mozilla::Side);
  inline T& Get(WritingMode, LogicalSide);
  inline T& GetIStart(WritingMode);
  inline T& GetBStart(WritingMode);
  inline T& GetIEnd(WritingMode);
  inline T& GetBEnd(WritingMode);
};

#if defined(CBINDGEN_IS_GECKO)
/// The value of an IntersectionObserver's rootMargin property.
///
/// Only bare px or percentage values are allowed. Other length units and
/// calc() values are not allowed.
///
/// <https://w3c.github.io/IntersectionObserver/#parse-a-root-margin>
using StyleIntersectionObserverRootMargin = StyleRect<StyleLengthPercentage>;
#endif

/// A wrapper of Non-negative values.
template<typename T>
using StyleNonNegative = T;

/// A wrapper of LengthPercentage, whose value must be >= 0.
using StyleNonNegativeLengthPercentage = StyleNonNegative<StyleLengthPercentage>;

/// A generic size, for `border-*-radius` longhand properties, or
/// `border-spacing`.
template<typename L>
struct StyleSize2D {
  L width;
  L height;

  bool operator==(const StyleSize2D& other) const {
    return width == other.width &&
           height == other.height;
  }
  bool operator!=(const StyleSize2D& other) const {
    return width != other.width ||
           height != other.height;
  }
};

/// A generic value for the `border-*-radius` longhand properties.
template<typename L>
struct StyleGenericBorderCornerRadius {
  StyleSize2D<L> _0;

  bool operator==(const StyleGenericBorderCornerRadius& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleGenericBorderCornerRadius& other) const {
    return _0 != other._0;
  }
};

/// A generic value for `border-radius`, `outline-radius` and `inset()`.
///
/// <https://drafts.csswg.org/css-backgrounds-3/#border-radius>
template<typename LengthPercentage>
struct StyleGenericBorderRadius {
  /// The top left radius.
  StyleGenericBorderCornerRadius<LengthPercentage> top_left;
  /// The top right radius.
  StyleGenericBorderCornerRadius<LengthPercentage> top_right;
  /// The bottom right radius.
  StyleGenericBorderCornerRadius<LengthPercentage> bottom_right;
  /// The bottom left radius.
  StyleGenericBorderCornerRadius<LengthPercentage> bottom_left;

  bool operator==(const StyleGenericBorderRadius& other) const {
    return top_left == other.top_left &&
           top_right == other.top_right &&
           bottom_right == other.bottom_right &&
           bottom_left == other.bottom_left;
  }
  bool operator!=(const StyleGenericBorderRadius& other) const {
    return top_left != other.top_left ||
           top_right != other.top_right ||
           bottom_right != other.bottom_right ||
           bottom_left != other.bottom_left;
  }
  inline const StyleLengthPercentage& Get(HalfCorner) const;
};

/// <https://drafts.csswg.org/css-shapes/#funcdef-inset>
template<typename LengthPercentage, typename NonNegativeLengthPercentage>
struct StyleInsetRect {
  StyleRect<LengthPercentage> rect;
  StyleGenericBorderRadius<NonNegativeLengthPercentage> round;

  bool operator==(const StyleInsetRect& other) const {
    return rect == other.rect &&
           round == other.round;
  }
  bool operator!=(const StyleInsetRect& other) const {
    return rect != other.rect ||
           round != other.round;
  }
};

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-radius>
template<typename NonNegativeLengthPercentage>
struct StyleGenericShapeRadius {
  enum class Tag : uint8_t {
    Length,
    ClosestSide,
    FarthestSide,
  };

  struct StyleLength_Body {
    NonNegativeLengthPercentage _0;

    bool operator==(const StyleLength_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLength_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleLength_Body length;
  };

  static StyleGenericShapeRadius Length(const NonNegativeLengthPercentage &_0) {
    StyleGenericShapeRadius result;
    ::new (&result.length._0) (NonNegativeLengthPercentage)(_0);
    result.tag = Tag::Length;
    return result;
  }

  bool IsLength() const {
    return tag == Tag::Length;
  }

  const NonNegativeLengthPercentage& AsLength() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLength());
    return length._0;
  }

  static StyleGenericShapeRadius ClosestSide() {
    StyleGenericShapeRadius result;
    result.tag = Tag::ClosestSide;
    return result;
  }

  bool IsClosestSide() const {
    return tag == Tag::ClosestSide;
  }

  static StyleGenericShapeRadius FarthestSide() {
    StyleGenericShapeRadius result;
    result.tag = Tag::FarthestSide;
    return result;
  }

  bool IsFarthestSide() const {
    return tag == Tag::FarthestSide;
  }

  bool operator==(const StyleGenericShapeRadius& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Length: return length == other.length;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericShapeRadius& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericShapeRadius() {

  }
  public:


  ~StyleGenericShapeRadius() {
    switch (tag) {
      case Tag::Length: length.~StyleLength_Body(); break;
      default: break;
    }
  }

  StyleGenericShapeRadius(const StyleGenericShapeRadius& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Length: ::new (&length) (StyleLength_Body)(other.length); break;
      default: break;
    }
  }
  StyleGenericShapeRadius& operator=(const StyleGenericShapeRadius& other) {
    if (this != &other) {
      this->~StyleGenericShapeRadius();
      new (this) StyleGenericShapeRadius(other);
    }
    return *this;
  }
};

/// <https://drafts.csswg.org/css-shapes/#funcdef-circle>
template<typename H, typename V, typename NonNegativeLengthPercentage>
struct StyleCircle {
  StyleGenericPosition<H, V> position;
  StyleGenericShapeRadius<NonNegativeLengthPercentage> radius;

  bool operator==(const StyleCircle& other) const {
    return position == other.position &&
           radius == other.radius;
  }
  bool operator!=(const StyleCircle& other) const {
    return position != other.position ||
           radius != other.radius;
  }
};

/// <https://drafts.csswg.org/css-shapes/#funcdef-ellipse>
template<typename H, typename V, typename NonNegativeLengthPercentage>
struct StyleEllipse {
  StyleGenericPosition<H, V> position;
  StyleGenericShapeRadius<NonNegativeLengthPercentage> semiaxis_x;
  StyleGenericShapeRadius<NonNegativeLengthPercentage> semiaxis_y;

  bool operator==(const StyleEllipse& other) const {
    return position == other.position &&
           semiaxis_x == other.semiaxis_x &&
           semiaxis_y == other.semiaxis_y;
  }
  bool operator!=(const StyleEllipse& other) const {
    return position != other.position ||
           semiaxis_x != other.semiaxis_x ||
           semiaxis_y != other.semiaxis_y;
  }
};

/// Coordinates for Polygon.
template<typename LengthPercentage>
struct StylePolygonCoord {
  LengthPercentage _0;
  LengthPercentage _1;

  bool operator==(const StylePolygonCoord& other) const {
    return _0 == other._0 &&
           _1 == other._1;
  }
  bool operator!=(const StylePolygonCoord& other) const {
    return _0 != other._0 ||
           _1 != other._1;
  }
};

/// A generic type for representing the `polygon()` function
///
/// <https://drafts.csswg.org/css-shapes/#funcdef-polygon>
template<typename LengthPercentage>
struct StyleGenericPolygon {
  /// The filling rule for a polygon.
  StyleFillRule fill;
  /// A collection of (x, y) coordinates to draw the polygon.
  StyleOwnedSlice<StylePolygonCoord<LengthPercentage>> coordinates;

  bool operator==(const StyleGenericPolygon& other) const {
    return fill == other.fill &&
           coordinates == other.coordinates;
  }
  bool operator!=(const StyleGenericPolygon& other) const {
    return fill != other.fill ||
           coordinates != other.coordinates;
  }
};

template<typename H, typename V, typename LengthPercentage, typename NonNegativeLengthPercentage>
struct StyleGenericBasicShape {
  enum class Tag : uint8_t {
    Inset,
    Circle,
    Ellipse,
    Polygon,
  };

  struct StyleInset_Body {
    StyleInsetRect<LengthPercentage, NonNegativeLengthPercentage> _0;

    bool operator==(const StyleInset_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleInset_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleCircle_Body {
    StyleCircle<H, V, NonNegativeLengthPercentage> _0;

    bool operator==(const StyleCircle_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleCircle_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleEllipse_Body {
    StyleEllipse<H, V, NonNegativeLengthPercentage> _0;

    bool operator==(const StyleEllipse_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleEllipse_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StylePolygon_Body {
    StyleGenericPolygon<LengthPercentage> _0;

    bool operator==(const StylePolygon_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StylePolygon_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleInset_Body inset;
    StyleCircle_Body circle;
    StyleEllipse_Body ellipse;
    StylePolygon_Body polygon;
  };

  static StyleGenericBasicShape Inset(const StyleInsetRect<LengthPercentage, NonNegativeLengthPercentage> &_0) {
    StyleGenericBasicShape result;
    ::new (&result.inset._0) (StyleInsetRect<LengthPercentage, NonNegativeLengthPercentage>)(_0);
    result.tag = Tag::Inset;
    return result;
  }

  bool IsInset() const {
    return tag == Tag::Inset;
  }

  const StyleInsetRect<LengthPercentage, NonNegativeLengthPercentage>& AsInset() const {
    MOZ_DIAGNOSTIC_ASSERT(IsInset());
    return inset._0;
  }

  static StyleGenericBasicShape Circle(const StyleCircle<H, V, NonNegativeLengthPercentage> &_0) {
    StyleGenericBasicShape result;
    ::new (&result.circle._0) (StyleCircle<H, V, NonNegativeLengthPercentage>)(_0);
    result.tag = Tag::Circle;
    return result;
  }

  bool IsCircle() const {
    return tag == Tag::Circle;
  }

  const StyleCircle<H, V, NonNegativeLengthPercentage>& AsCircle() const {
    MOZ_DIAGNOSTIC_ASSERT(IsCircle());
    return circle._0;
  }

  static StyleGenericBasicShape Ellipse(const StyleEllipse<H, V, NonNegativeLengthPercentage> &_0) {
    StyleGenericBasicShape result;
    ::new (&result.ellipse._0) (StyleEllipse<H, V, NonNegativeLengthPercentage>)(_0);
    result.tag = Tag::Ellipse;
    return result;
  }

  bool IsEllipse() const {
    return tag == Tag::Ellipse;
  }

  const StyleEllipse<H, V, NonNegativeLengthPercentage>& AsEllipse() const {
    MOZ_DIAGNOSTIC_ASSERT(IsEllipse());
    return ellipse._0;
  }

  static StyleGenericBasicShape Polygon(const StyleGenericPolygon<LengthPercentage> &_0) {
    StyleGenericBasicShape result;
    ::new (&result.polygon._0) (StyleGenericPolygon<LengthPercentage>)(_0);
    result.tag = Tag::Polygon;
    return result;
  }

  bool IsPolygon() const {
    return tag == Tag::Polygon;
  }

  const StyleGenericPolygon<LengthPercentage>& AsPolygon() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPolygon());
    return polygon._0;
  }

  bool operator==(const StyleGenericBasicShape& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Inset: return inset == other.inset;
      case Tag::Circle: return circle == other.circle;
      case Tag::Ellipse: return ellipse == other.ellipse;
      case Tag::Polygon: return polygon == other.polygon;

    }
    return true;
  }

  bool operator!=(const StyleGenericBasicShape& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericBasicShape() {

  }
  public:


  ~StyleGenericBasicShape() {
    switch (tag) {
      case Tag::Inset: inset.~StyleInset_Body(); break;
      case Tag::Circle: circle.~StyleCircle_Body(); break;
      case Tag::Ellipse: ellipse.~StyleEllipse_Body(); break;
      case Tag::Polygon: polygon.~StylePolygon_Body(); break;

    }
  }

  StyleGenericBasicShape(const StyleGenericBasicShape& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Inset: ::new (&inset) (StyleInset_Body)(other.inset); break;
      case Tag::Circle: ::new (&circle) (StyleCircle_Body)(other.circle); break;
      case Tag::Ellipse: ::new (&ellipse) (StyleEllipse_Body)(other.ellipse); break;
      case Tag::Polygon: ::new (&polygon) (StylePolygon_Body)(other.polygon); break;

    }
  }
  StyleGenericBasicShape& operator=(const StyleGenericBasicShape& other) {
    if (this != &other) {
      this->~StyleGenericBasicShape();
      new (this) StyleGenericBasicShape(other);
    }
    return *this;
  }
};

/// A computed basic shape.
using StyleBasicShape = StyleGenericBasicShape<StyleLengthPercentage, StyleLengthPercentage, StyleLengthPercentage, StyleNonNegativeLengthPercentage>;

/// <https://drafts.csswg.org/css-ui/#propdef-outline-style>
struct StyleOutlineStyle {
  enum class Tag : uint8_t {
    /// auto
    Auto,
    /// <border-style>
    BorderStyle,
  };

  struct StyleBorderStyle_Body {
    StyleBorderStyle _0;

    bool operator==(const StyleBorderStyle_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleBorderStyle_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleBorderStyle_Body border_style;
  };

  static StyleOutlineStyle Auto() {
    StyleOutlineStyle result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  static StyleOutlineStyle BorderStyle(const StyleBorderStyle &_0) {
    StyleOutlineStyle result;
    ::new (&result.border_style._0) (StyleBorderStyle)(_0);
    result.tag = Tag::BorderStyle;
    return result;
  }

  bool IsBorderStyle() const {
    return tag == Tag::BorderStyle;
  }

  const StyleBorderStyle& AsBorderStyle() const {
    MOZ_DIAGNOSTIC_ASSERT(IsBorderStyle());
    return border_style._0;
  }

  bool operator==(const StyleOutlineStyle& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::BorderStyle: return border_style == other.border_style;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleOutlineStyle& other) const {
    return !(*this == other);
  }

  private:
  StyleOutlineStyle() {

  }
  public:


  ~StyleOutlineStyle() {
    switch (tag) {
      case Tag::BorderStyle: border_style.~StyleBorderStyle_Body(); break;
      default: break;
    }
  }

  StyleOutlineStyle(const StyleOutlineStyle& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::BorderStyle: ::new (&border_style) (StyleBorderStyle_Body)(other.border_style); break;
      default: break;
    }
  }
  StyleOutlineStyle& operator=(const StyleOutlineStyle& other) {
    if (this != &other) {
      this->~StyleOutlineStyle();
      new (this) StyleOutlineStyle(other);
    }
    return *this;
  }
};

/// A generic easing function.
template<typename Integer, typename Number>
struct StyleTimingFunction {
  enum class Tag : uint8_t {
    /// `linear | ease | ease-in | ease-out | ease-in-out`
    Keyword,
    /// `cubic-bezier(<number>, <number>, <number>, <number>)`
    CubicBezier,
    /// `step-start | step-end | steps(<integer>, [ <step-position> ]?)`
    /// `<step-position> = jump-start | jump-end | jump-none | jump-both | start | end`
    Steps,
  };

  struct StyleKeyword_Body {
    StyleTimingKeyword _0;

    bool operator==(const StyleKeyword_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleKeyword_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleCubicBezier_Body {
    Number x1;
    Number y1;
    Number x2;
    Number y2;

    bool operator==(const StyleCubicBezier_Body& other) const {
      return x1 == other.x1 &&
             y1 == other.y1 &&
             x2 == other.x2 &&
             y2 == other.y2;
    }
    bool operator!=(const StyleCubicBezier_Body& other) const {
      return x1 != other.x1 ||
             y1 != other.y1 ||
             x2 != other.x2 ||
             y2 != other.y2;
    }
  };

  struct StyleSteps_Body {
    Integer _0;
    StyleStepPosition _1;

    bool operator==(const StyleSteps_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const StyleSteps_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  Tag tag;
  union {
    StyleKeyword_Body keyword;
    StyleCubicBezier_Body cubic_bezier;
    StyleSteps_Body steps;
  };

  static StyleTimingFunction Keyword(const StyleTimingKeyword &_0) {
    StyleTimingFunction result;
    ::new (&result.keyword._0) (StyleTimingKeyword)(_0);
    result.tag = Tag::Keyword;
    return result;
  }

  bool IsKeyword() const {
    return tag == Tag::Keyword;
  }

  const StyleTimingKeyword& AsKeyword() const {
    MOZ_DIAGNOSTIC_ASSERT(IsKeyword());
    return keyword._0;
  }

  static StyleTimingFunction CubicBezier(const Number &x1,
                                         const Number &y1,
                                         const Number &x2,
                                         const Number &y2) {
    StyleTimingFunction result;
    ::new (&result.cubic_bezier.x1) (Number)(x1);
    ::new (&result.cubic_bezier.y1) (Number)(y1);
    ::new (&result.cubic_bezier.x2) (Number)(x2);
    ::new (&result.cubic_bezier.y2) (Number)(y2);
    result.tag = Tag::CubicBezier;
    return result;
  }

  bool IsCubicBezier() const {
    return tag == Tag::CubicBezier;
  }

  const StyleCubicBezier_Body& AsCubicBezier() const {
    MOZ_DIAGNOSTIC_ASSERT(IsCubicBezier());
    return cubic_bezier;
  }

  static StyleTimingFunction Steps(const Integer &_0,
                                   const StyleStepPosition &_1) {
    StyleTimingFunction result;
    ::new (&result.steps._0) (Integer)(_0);
    ::new (&result.steps._1) (StyleStepPosition)(_1);
    result.tag = Tag::Steps;
    return result;
  }

  bool IsSteps() const {
    return tag == Tag::Steps;
  }

  const StyleSteps_Body& AsSteps() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSteps());
    return steps;
  }

  bool operator==(const StyleTimingFunction& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Keyword: return keyword == other.keyword;
      case Tag::CubicBezier: return cubic_bezier == other.cubic_bezier;
      case Tag::Steps: return steps == other.steps;

    }
    return true;
  }

  bool operator!=(const StyleTimingFunction& other) const {
    return !(*this == other);
  }

  private:
  StyleTimingFunction() {

  }
  public:


  ~StyleTimingFunction() {
    switch (tag) {
      case Tag::Keyword: keyword.~StyleKeyword_Body(); break;
      case Tag::CubicBezier: cubic_bezier.~StyleCubicBezier_Body(); break;
      case Tag::Steps: steps.~StyleSteps_Body(); break;

    }
  }

  StyleTimingFunction(const StyleTimingFunction& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Keyword: ::new (&keyword) (StyleKeyword_Body)(other.keyword); break;
      case Tag::CubicBezier: ::new (&cubic_bezier) (StyleCubicBezier_Body)(other.cubic_bezier); break;
      case Tag::Steps: ::new (&steps) (StyleSteps_Body)(other.steps); break;

    }
  }
  StyleTimingFunction& operator=(const StyleTimingFunction& other) {
    if (this != &other) {
      this->~StyleTimingFunction();
      new (this) StyleTimingFunction(other);
    }
    return *this;
  }
};

/// A computed timing function.
using StyleComputedTimingFunction = StyleTimingFunction<StyleInteger, StyleNumber>;

/// Misc information about a given computed style.
///
/// All flags are currently inherited for text, pseudo elements, and
/// anonymous boxes, see StyleBuilder::for_inheritance and its callsites.
/// If we ever want to add some flags that shouldn't inherit for them,
/// we might want to add a function to handle this.
struct StyleComputedValueFlags {
  uint16_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleComputedValueFlags operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleComputedValueFlags operator|(const StyleComputedValueFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleComputedValueFlags& operator|=(const StyleComputedValueFlags& other) {
    *this = (*this | other);
    return *this;
  }
  StyleComputedValueFlags operator&(const StyleComputedValueFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleComputedValueFlags& operator&=(const StyleComputedValueFlags& other) {
    *this = (*this & other);
    return *this;
  }
  StyleComputedValueFlags operator^(const StyleComputedValueFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleComputedValueFlags& operator^=(const StyleComputedValueFlags& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleComputedValueFlags& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleComputedValueFlags& other) const {
    return bits != other.bits;
  }
  static const StyleComputedValueFlags HAS_TEXT_DECORATION_LINES;
  static const StyleComputedValueFlags SHOULD_SUPPRESS_LINEBREAK;
  static const StyleComputedValueFlags IS_TEXT_COMBINED;
  static const StyleComputedValueFlags IS_RELEVANT_LINK_VISITED;
  static const StyleComputedValueFlags IS_IN_PSEUDO_ELEMENT_SUBTREE;
  static const StyleComputedValueFlags DISPLAY_DEPENDS_ON_INHERITED_STYLE;
  static const StyleComputedValueFlags CONTENT_DEPENDS_ON_INHERITED_STYLE;
  static const StyleComputedValueFlags INHERITS_RESET_STYLE;
  static const StyleComputedValueFlags DEPENDS_ON_SELF_FONT_METRICS;
  static const StyleComputedValueFlags DEPENDS_ON_INHERITED_FONT_METRICS;
  static const StyleComputedValueFlags CAN_BE_FRAGMENTED;
  static const StyleComputedValueFlags IS_ROOT_ELEMENT_STYLE;
  static const StyleComputedValueFlags IS_IN_OPACITY_ZERO_SUBTREE;
  static const StyleComputedValueFlags HAS_AUTHOR_SPECIFIED_BORDER_BACKGROUND;
  static const StyleComputedValueFlags HAS_AUTHOR_SPECIFIED_PADDING;
};
/// Whether the style or any of the ancestors has a text-decoration-line
/// property that should get propagated to descendants.
///
/// text-decoration-line is a reset property, but gets propagated in the
/// frame/box tree.
inline const StyleComputedValueFlags StyleComputedValueFlags::HAS_TEXT_DECORATION_LINES = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 0) };
/// Whether line break inside should be suppressed.
///
/// If this flag is set, the line should not be broken inside,
/// which means inlines act as if nowrap is set, <br> element is
/// suppressed, and blocks are inlinized.
///
/// This bit is propagated to all children of line participants.
/// It is currently used by ruby to make its content unbreakable.
inline const StyleComputedValueFlags StyleComputedValueFlags::SHOULD_SUPPRESS_LINEBREAK = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 1) };
/// A flag used to mark text that that has text-combine-upright.
///
/// This is used from Gecko's layout engine.
inline const StyleComputedValueFlags StyleComputedValueFlags::IS_TEXT_COMBINED = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 2) };
/// A flag used to mark styles under a relevant link that is also
/// visited.
inline const StyleComputedValueFlags StyleComputedValueFlags::IS_RELEVANT_LINK_VISITED = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 3) };
/// A flag used to mark styles which are a pseudo-element or under one.
inline const StyleComputedValueFlags StyleComputedValueFlags::IS_IN_PSEUDO_ELEMENT_SUBTREE = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 4) };
/// Whether this style's `display` property depends on our parent style.
///
/// This is important because it may affect our optimizations to avoid
/// computing the style of pseudo-elements, given whether the
/// pseudo-element is generated depends on the `display` value.
inline const StyleComputedValueFlags StyleComputedValueFlags::DISPLAY_DEPENDS_ON_INHERITED_STYLE = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 6) };
/// Whether this style's `content` depends on our parent style.
///
/// Important because of the same reason.
inline const StyleComputedValueFlags StyleComputedValueFlags::CONTENT_DEPENDS_ON_INHERITED_STYLE = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 7) };
/// Whether the child explicitly inherits any reset property.
inline const StyleComputedValueFlags StyleComputedValueFlags::INHERITS_RESET_STYLE = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 8) };
/// Whether any value on our style is font-metric-dependent on our
/// primary font.
inline const StyleComputedValueFlags StyleComputedValueFlags::DEPENDS_ON_SELF_FONT_METRICS = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 9) };
/// Whether any value on our style is font-metric-dependent on the
/// primary font of our parent.
inline const StyleComputedValueFlags StyleComputedValueFlags::DEPENDS_ON_INHERITED_FONT_METRICS = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 10) };
/// Whether the style or any of the ancestors has a multicol style.
///
/// Only used in Servo.
inline const StyleComputedValueFlags StyleComputedValueFlags::CAN_BE_FRAGMENTED = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 11) };
/// Whether this style is the style of the document element.
inline const StyleComputedValueFlags StyleComputedValueFlags::IS_ROOT_ELEMENT_STYLE = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 12) };
/// Whether this element is inside an `opacity: 0` subtree.
inline const StyleComputedValueFlags StyleComputedValueFlags::IS_IN_OPACITY_ZERO_SUBTREE = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 13) };
/// Whether there are author-specified rules for border-* properties
/// (except border-image-*), background-color, or background-image.
///
/// TODO(emilio): Maybe do include border-image, see:
///
/// https://github.com/w3c/csswg-drafts/issues/4777#issuecomment-604424845
inline const StyleComputedValueFlags StyleComputedValueFlags::HAS_AUTHOR_SPECIFIED_BORDER_BACKGROUND = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 14) };
/// Whether there are author-specified rules for padding-* properties.
///
/// FIXME(emilio): Try to merge this with BORDER_BACKGROUND, see
/// https://github.com/w3c/csswg-drafts/issues/4777
inline const StyleComputedValueFlags StyleComputedValueFlags::HAS_AUTHOR_SPECIFIED_PADDING = StyleComputedValueFlags{ /* .bits = */ (uint16_t)(1 << 15) };

/// https://drafts.csswg.org/css-scroll-snap-1/#scroll-snap-align
struct StyleScrollSnapAlign {
  StyleScrollSnapAlignKeyword block;
  StyleScrollSnapAlignKeyword inline_;

  bool operator==(const StyleScrollSnapAlign& other) const {
    return block == other.block &&
           inline_ == other.inline_;
  }
  bool operator!=(const StyleScrollSnapAlign& other) const {
    return block != other.block ||
           inline_ != other.inline_;
  }
};

/// https://drafts.csswg.org/css-scroll-snap-1/#scroll-snap-type
struct StyleScrollSnapType {
  StyleScrollSnapAxis axis;
  StyleScrollSnapStrictness strictness;

  bool operator==(const StyleScrollSnapType& other) const {
    return axis == other.axis &&
           strictness == other.strictness;
  }
  bool operator!=(const StyleScrollSnapType& other) const {
    return axis != other.axis ||
           strictness != other.strictness;
  }
};

/// A computed value for the `letter-spacing` property.
using StyleLetterSpacing = StyleLength;

/// A `<length-percentage> | auto` value.
template<typename LengthPercent>
struct StyleGenericLengthPercentageOrAuto {
  enum class Tag : uint8_t {
    LengthPercentage,
    Auto,
  };

  struct StyleLengthPercentage_Body {
    LengthPercent _0;

    bool operator==(const StyleLengthPercentage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLengthPercentage_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleLengthPercentage_Body length_percentage;
  };

  static StyleGenericLengthPercentageOrAuto LengthPercentage(const LengthPercent &_0) {
    StyleGenericLengthPercentageOrAuto result;
    ::new (&result.length_percentage._0) (LengthPercent)(_0);
    result.tag = Tag::LengthPercentage;
    return result;
  }

  bool IsLengthPercentage() const {
    return tag == Tag::LengthPercentage;
  }

  const LengthPercent& AsLengthPercentage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLengthPercentage());
    return length_percentage._0;
  }

  static StyleGenericLengthPercentageOrAuto Auto() {
    StyleGenericLengthPercentageOrAuto result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  bool operator==(const StyleGenericLengthPercentageOrAuto& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::LengthPercentage: return length_percentage == other.length_percentage;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericLengthPercentageOrAuto& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericLengthPercentageOrAuto() {

  }
  public:


  ~StyleGenericLengthPercentageOrAuto() {
    switch (tag) {
      case Tag::LengthPercentage: length_percentage.~StyleLengthPercentage_Body(); break;
      default: break;
    }
  }

  StyleGenericLengthPercentageOrAuto(const StyleGenericLengthPercentageOrAuto& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::LengthPercentage: ::new (&length_percentage) (StyleLengthPercentage_Body)(other.length_percentage); break;
      default: break;
    }
  }
  StyleGenericLengthPercentageOrAuto& operator=(const StyleGenericLengthPercentageOrAuto& other) {
    if (this != &other) {
      this->~StyleGenericLengthPercentageOrAuto();
      new (this) StyleGenericLengthPercentageOrAuto(other);
    }
    return *this;
  }
  inline bool ConvertsToLength() const;
  inline nscoord ToLength() const;
  inline bool ConvertsToPercentage() const;
  inline float ToPercentage() const;
  inline bool HasPercent() const;
  inline bool HasLengthAndPercentage() const;

  // Just some convenient aliases for LengthOrAuto, to avoid confusing naming.
  inline bool IsLength() const;
  inline const StyleLength& AsLength() const;
};

/// A computed type for `<length-percentage> | auto`.
using StyleLengthPercentageOrAuto = StyleGenericLengthPercentageOrAuto<StyleLengthPercentage>;

/// A wrapper of Number, but the value >= 0.
using StyleNonNegativeNumber = StyleNonNegative<StyleCSSFloat>;

/// A wrapper of Length, whose value must be >= 0.
using StyleNonNegativeLength = StyleNonNegative<StyleLength>;

/// A generic value for the `line-height` property.
template<typename N, typename L>
struct StyleGenericLineHeight {
  enum class Tag : uint8_t {
    /// `normal`
    Normal,
#if defined(CBINDGEN_IS_GECKO)
    /// `-moz-block-height`
    MozBlockHeight,
#endif
    /// `<number>`
    Number,
    /// `<length-percentage>`
    Length,
  };

  struct StyleNumber_Body {
    N _0;

    bool operator==(const StyleNumber_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleNumber_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleLength_Body {
    L _0;

    bool operator==(const StyleLength_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLength_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleNumber_Body number;
    StyleLength_Body length;
  };

  static StyleGenericLineHeight Normal() {
    StyleGenericLineHeight result;
    result.tag = Tag::Normal;
    return result;
  }

  bool IsNormal() const {
    return tag == Tag::Normal;
  }

#if defined(CBINDGEN_IS_GECKO)
  static StyleGenericLineHeight MozBlockHeight() {
    StyleGenericLineHeight result;
    result.tag = Tag::MozBlockHeight;
    return result;
  }

  bool IsMozBlockHeight() const {
    return tag == Tag::MozBlockHeight;
  }
#endif

  static StyleGenericLineHeight Number(const N &_0) {
    StyleGenericLineHeight result;
    ::new (&result.number._0) (N)(_0);
    result.tag = Tag::Number;
    return result;
  }

  bool IsNumber() const {
    return tag == Tag::Number;
  }

  const N& AsNumber() const {
    MOZ_DIAGNOSTIC_ASSERT(IsNumber());
    return number._0;
  }

  static StyleGenericLineHeight Length(const L &_0) {
    StyleGenericLineHeight result;
    ::new (&result.length._0) (L)(_0);
    result.tag = Tag::Length;
    return result;
  }

  bool IsLength() const {
    return tag == Tag::Length;
  }

  const L& AsLength() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLength());
    return length._0;
  }

  bool operator==(const StyleGenericLineHeight& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Number: return number == other.number;
      case Tag::Length: return length == other.length;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericLineHeight& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericLineHeight() {

  }
  public:


  ~StyleGenericLineHeight() {
    switch (tag) {
      case Tag::Number: number.~StyleNumber_Body(); break;
      case Tag::Length: length.~StyleLength_Body(); break;
      default: break;
    }
  }

  StyleGenericLineHeight(const StyleGenericLineHeight& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Number: ::new (&number) (StyleNumber_Body)(other.number); break;
      case Tag::Length: ::new (&length) (StyleLength_Body)(other.length); break;
      default: break;
    }
  }
  StyleGenericLineHeight& operator=(const StyleGenericLineHeight& other) {
    if (this != &other) {
      this->~StyleGenericLineHeight();
      new (this) StyleGenericLineHeight(other);
    }
    return *this;
  }
};

/// A computed value for the `line-height` property.
using StyleLineHeight = StyleGenericLineHeight<StyleNonNegativeNumber, StyleNonNegativeLength>;

/// A wrapper of LengthPercentageOrAuto, whose value must be >= 0.
using StyleNonNegativeLengthPercentageOrAuto = StyleGenericLengthPercentageOrAuto<StyleNonNegativeLengthPercentage>;

/// A generic `<length-percentage>` | normal` value.
template<typename LengthPercent>
struct StyleGenericLengthPercentageOrNormal {
  enum class Tag : uint8_t {
    LengthPercentage,
    Normal,
  };

  struct StyleLengthPercentage_Body {
    LengthPercent _0;

    bool operator==(const StyleLengthPercentage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLengthPercentage_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleLengthPercentage_Body length_percentage;
  };

  static StyleGenericLengthPercentageOrNormal LengthPercentage(const LengthPercent &_0) {
    StyleGenericLengthPercentageOrNormal result;
    ::new (&result.length_percentage._0) (LengthPercent)(_0);
    result.tag = Tag::LengthPercentage;
    return result;
  }

  bool IsLengthPercentage() const {
    return tag == Tag::LengthPercentage;
  }

  const LengthPercent& AsLengthPercentage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLengthPercentage());
    return length_percentage._0;
  }

  static StyleGenericLengthPercentageOrNormal Normal() {
    StyleGenericLengthPercentageOrNormal result;
    result.tag = Tag::Normal;
    return result;
  }

  bool IsNormal() const {
    return tag == Tag::Normal;
  }

  bool operator==(const StyleGenericLengthPercentageOrNormal& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::LengthPercentage: return length_percentage == other.length_percentage;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericLengthPercentageOrNormal& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericLengthPercentageOrNormal() {

  }
  public:


  ~StyleGenericLengthPercentageOrNormal() {
    switch (tag) {
      case Tag::LengthPercentage: length_percentage.~StyleLengthPercentage_Body(); break;
      default: break;
    }
  }

  StyleGenericLengthPercentageOrNormal(const StyleGenericLengthPercentageOrNormal& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::LengthPercentage: ::new (&length_percentage) (StyleLengthPercentage_Body)(other.length_percentage); break;
      default: break;
    }
  }
  StyleGenericLengthPercentageOrNormal& operator=(const StyleGenericLengthPercentageOrNormal& other) {
    if (this != &other) {
      this->~StyleGenericLengthPercentageOrNormal();
      new (this) StyleGenericLengthPercentageOrNormal(other);
    }
    return *this;
  }
};

/// Either a computed NonNegativeLengthPercentage or the `normal` keyword.
using StyleNonNegativeLengthPercentageOrNormal = StyleGenericLengthPercentageOrNormal<StyleNonNegativeLengthPercentage>;

/// Either a computed `<length>` or the `auto` keyword.
using StyleLengthOrAuto = StyleGenericLengthPercentageOrAuto<StyleLength>;

/// Either a non-negative `<length>` or the `auto` keyword.
using StyleNonNegativeLengthOrAuto = StyleGenericLengthPercentageOrAuto<StyleNonNegativeLength>;

/// Implements type for text-decoration-thickness
/// which takes the grammar of auto | from-font | <length> | <percentage>
///
/// https://drafts.csswg.org/css-text-decor-4/
template<typename L>
struct StyleGenericTextDecorationLength {
  enum class Tag : uint8_t {
    LengthPercentage,
    Auto,
    FromFont,
  };

  struct StyleLengthPercentage_Body {
    L _0;

    bool operator==(const StyleLengthPercentage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLengthPercentage_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleLengthPercentage_Body length_percentage;
  };

  static StyleGenericTextDecorationLength LengthPercentage(const L &_0) {
    StyleGenericTextDecorationLength result;
    ::new (&result.length_percentage._0) (L)(_0);
    result.tag = Tag::LengthPercentage;
    return result;
  }

  bool IsLengthPercentage() const {
    return tag == Tag::LengthPercentage;
  }

  const L& AsLengthPercentage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLengthPercentage());
    return length_percentage._0;
  }

  static StyleGenericTextDecorationLength Auto() {
    StyleGenericTextDecorationLength result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  static StyleGenericTextDecorationLength FromFont() {
    StyleGenericTextDecorationLength result;
    result.tag = Tag::FromFont;
    return result;
  }

  bool IsFromFont() const {
    return tag == Tag::FromFont;
  }

  bool operator==(const StyleGenericTextDecorationLength& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::LengthPercentage: return length_percentage == other.length_percentage;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericTextDecorationLength& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericTextDecorationLength() {

  }
  public:


  ~StyleGenericTextDecorationLength() {
    switch (tag) {
      case Tag::LengthPercentage: length_percentage.~StyleLengthPercentage_Body(); break;
      default: break;
    }
  }

  StyleGenericTextDecorationLength(const StyleGenericTextDecorationLength& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::LengthPercentage: ::new (&length_percentage) (StyleLengthPercentage_Body)(other.length_percentage); break;
      default: break;
    }
  }
  StyleGenericTextDecorationLength& operator=(const StyleGenericTextDecorationLength& other) {
    if (this != &other) {
      this->~StyleGenericTextDecorationLength();
      new (this) StyleGenericTextDecorationLength(other);
    }
    return *this;
  }
};

/// Implements type for `text-decoration-thickness` property.
using StyleTextDecorationLength = StyleGenericTextDecorationLength<StyleLengthPercentage>;

/// A generic value for the `width`, `height`, `min-width`, or `min-height` property.
///
/// Unlike `max-width` or `max-height` properties, a Size can be `auto`,
/// and cannot be `none`.
///
/// Note that it only accepts non-negative values.
template<typename LengthPercent>
struct StyleGenericSize {
  enum class Tag : uint8_t {
    LengthPercentage,
    Auto,
#if defined(CBINDGEN_IS_GECKO)
    ExtremumLength,
#endif
  };

  struct StyleLengthPercentage_Body {
    LengthPercent _0;

    bool operator==(const StyleLengthPercentage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLengthPercentage_Body& other) const {
      return _0 != other._0;
    }
  };

#if defined(CBINDGEN_IS_GECKO)
  struct StyleExtremumLength_Body {
    StyleExtremumLength _0;

    bool operator==(const StyleExtremumLength_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleExtremumLength_Body& other) const {
      return _0 != other._0;
    }
  };
#endif

  Tag tag;
  union {
    StyleLengthPercentage_Body length_percentage;
#if defined(CBINDGEN_IS_GECKO)
    StyleExtremumLength_Body extremum_length;
#endif
  };

  static StyleGenericSize LengthPercentage(const LengthPercent &_0) {
    StyleGenericSize result;
    ::new (&result.length_percentage._0) (LengthPercent)(_0);
    result.tag = Tag::LengthPercentage;
    return result;
  }

  bool IsLengthPercentage() const {
    return tag == Tag::LengthPercentage;
  }

  const LengthPercent& AsLengthPercentage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLengthPercentage());
    return length_percentage._0;
  }

  static StyleGenericSize Auto() {
    StyleGenericSize result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

#if defined(CBINDGEN_IS_GECKO)
  static StyleGenericSize ExtremumLength(const StyleExtremumLength &_0) {
    StyleGenericSize result;
    ::new (&result.extremum_length._0) (StyleExtremumLength)(_0);
    result.tag = Tag::ExtremumLength;
    return result;
  }

  bool IsExtremumLength() const {
    return tag == Tag::ExtremumLength;
  }

  const StyleExtremumLength& AsExtremumLength() const {
    MOZ_DIAGNOSTIC_ASSERT(IsExtremumLength());
    return extremum_length._0;
  }
#endif

  bool operator==(const StyleGenericSize& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::LengthPercentage: return length_percentage == other.length_percentage;
#if defined(CBINDGEN_IS_GECKO)
      case Tag::ExtremumLength: return extremum_length == other.extremum_length;
#endif
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericSize& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericSize() {

  }
  public:


  ~StyleGenericSize() {
    switch (tag) {
      case Tag::LengthPercentage: length_percentage.~StyleLengthPercentage_Body(); break;
#if defined(CBINDGEN_IS_GECKO)
      case Tag::ExtremumLength: extremum_length.~StyleExtremumLength_Body(); break;
#endif
      default: break;
    }
  }

  StyleGenericSize(const StyleGenericSize& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::LengthPercentage: ::new (&length_percentage) (StyleLengthPercentage_Body)(other.length_percentage); break;
#if defined(CBINDGEN_IS_GECKO)
      case Tag::ExtremumLength: ::new (&extremum_length) (StyleExtremumLength_Body)(other.extremum_length); break;
#endif
      default: break;
    }
  }
  StyleGenericSize& operator=(const StyleGenericSize& other) {
    if (this != &other) {
      this->~StyleGenericSize();
      new (this) StyleGenericSize(other);
    }
    return *this;
  }
  inline bool ConvertsToLength() const;
  inline nscoord ToLength() const;
  inline bool ConvertsToPercentage() const;
  inline float ToPercentage() const;
  inline bool HasPercent() const;
  inline bool HasLengthAndPercentage() const;
  inline bool BehavesLikeInitialValueOnBlockAxis() const;
};

/// A computed value for `min-width`, `min-height`, `width` or `height` property.
using StyleSize = StyleGenericSize<StyleNonNegativeLengthPercentage>;

/// A generic value for the `max-width` or `max-height` property.
template<typename LengthPercent>
struct StyleGenericMaxSize {
  enum class Tag : uint8_t {
    LengthPercentage,
    None,
#if defined(CBINDGEN_IS_GECKO)
    ExtremumLength,
#endif
  };

  struct StyleLengthPercentage_Body {
    LengthPercent _0;

    bool operator==(const StyleLengthPercentage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLengthPercentage_Body& other) const {
      return _0 != other._0;
    }
  };

#if defined(CBINDGEN_IS_GECKO)
  struct StyleExtremumLength_Body {
    StyleExtremumLength _0;

    bool operator==(const StyleExtremumLength_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleExtremumLength_Body& other) const {
      return _0 != other._0;
    }
  };
#endif

  Tag tag;
  union {
    StyleLengthPercentage_Body length_percentage;
#if defined(CBINDGEN_IS_GECKO)
    StyleExtremumLength_Body extremum_length;
#endif
  };

  static StyleGenericMaxSize LengthPercentage(const LengthPercent &_0) {
    StyleGenericMaxSize result;
    ::new (&result.length_percentage._0) (LengthPercent)(_0);
    result.tag = Tag::LengthPercentage;
    return result;
  }

  bool IsLengthPercentage() const {
    return tag == Tag::LengthPercentage;
  }

  const LengthPercent& AsLengthPercentage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLengthPercentage());
    return length_percentage._0;
  }

  static StyleGenericMaxSize None() {
    StyleGenericMaxSize result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

#if defined(CBINDGEN_IS_GECKO)
  static StyleGenericMaxSize ExtremumLength(const StyleExtremumLength &_0) {
    StyleGenericMaxSize result;
    ::new (&result.extremum_length._0) (StyleExtremumLength)(_0);
    result.tag = Tag::ExtremumLength;
    return result;
  }

  bool IsExtremumLength() const {
    return tag == Tag::ExtremumLength;
  }

  const StyleExtremumLength& AsExtremumLength() const {
    MOZ_DIAGNOSTIC_ASSERT(IsExtremumLength());
    return extremum_length._0;
  }
#endif

  bool operator==(const StyleGenericMaxSize& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::LengthPercentage: return length_percentage == other.length_percentage;
#if defined(CBINDGEN_IS_GECKO)
      case Tag::ExtremumLength: return extremum_length == other.extremum_length;
#endif
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericMaxSize& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericMaxSize() {

  }
  public:


  ~StyleGenericMaxSize() {
    switch (tag) {
      case Tag::LengthPercentage: length_percentage.~StyleLengthPercentage_Body(); break;
#if defined(CBINDGEN_IS_GECKO)
      case Tag::ExtremumLength: extremum_length.~StyleExtremumLength_Body(); break;
#endif
      default: break;
    }
  }

  StyleGenericMaxSize(const StyleGenericMaxSize& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::LengthPercentage: ::new (&length_percentage) (StyleLengthPercentage_Body)(other.length_percentage); break;
#if defined(CBINDGEN_IS_GECKO)
      case Tag::ExtremumLength: ::new (&extremum_length) (StyleExtremumLength_Body)(other.extremum_length); break;
#endif
      default: break;
    }
  }
  StyleGenericMaxSize& operator=(const StyleGenericMaxSize& other) {
    if (this != &other) {
      this->~StyleGenericMaxSize();
      new (this) StyleGenericMaxSize(other);
    }
    return *this;
  }
  inline bool ConvertsToLength() const;
  inline nscoord ToLength() const;
  inline bool ConvertsToPercentage() const;
  inline float ToPercentage() const;
  inline bool HasPercent() const;
  inline bool HasLengthAndPercentage() const;
  inline bool BehavesLikeInitialValueOnBlockAxis() const;
};

/// A computed value for `max-width` or `min-height` property.
using StyleMaxSize = StyleGenericMaxSize<StyleNonNegativeLengthPercentage>;

/// A generic value for the `flex-basis` property.
template<typename S>
struct StyleGenericFlexBasis {
  enum class Tag {
    /// `content`
    Content,
    /// `<width>`
    Size,
  };

  struct StyleSize_Body {
    S _0;

    bool operator==(const StyleSize_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleSize_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleSize_Body size;
  };

  static StyleGenericFlexBasis Content() {
    StyleGenericFlexBasis result;
    result.tag = Tag::Content;
    return result;
  }

  bool IsContent() const {
    return tag == Tag::Content;
  }

  static StyleGenericFlexBasis Size(const S &_0) {
    StyleGenericFlexBasis result;
    ::new (&result.size._0) (S)(_0);
    result.tag = Tag::Size;
    return result;
  }

  bool IsSize() const {
    return tag == Tag::Size;
  }

  const S& AsSize() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSize());
    return size._0;
  }

  bool operator==(const StyleGenericFlexBasis& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Size: return size == other.size;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericFlexBasis& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericFlexBasis() {

  }
  public:


  ~StyleGenericFlexBasis() {
    switch (tag) {
      case Tag::Size: size.~StyleSize_Body(); break;
      default: break;
    }
  }

  StyleGenericFlexBasis(const StyleGenericFlexBasis& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Size: ::new (&size) (StyleSize_Body)(other.size); break;
      default: break;
    }
  }
  StyleGenericFlexBasis& operator=(const StyleGenericFlexBasis& other) {
    if (this != &other) {
      this->~StyleGenericFlexBasis();
      new (this) StyleGenericFlexBasis(other);
    }
    return *this;
  }
  inline bool IsAuto() const;
};

/// A computed value for the `flex-basis` property.
using StyleFlexBasis = StyleGenericFlexBasis<StyleSize>;

/// A generic value for the `background-size` property.
template<typename LengthPercent>
struct StyleGenericBackgroundSize {
  enum class Tag : uint8_t {
    /// `<width> <height>`
    ExplicitSize,
    /// `cover`
    Cover,
    /// `contain`
    Contain,
  };

  struct StyleExplicitSize_Body {
    /// Explicit width.
    StyleGenericLengthPercentageOrAuto<LengthPercent> width;
    /// Explicit height.
    StyleGenericLengthPercentageOrAuto<LengthPercent> height;

    bool operator==(const StyleExplicitSize_Body& other) const {
      return width == other.width &&
             height == other.height;
    }
    bool operator!=(const StyleExplicitSize_Body& other) const {
      return width != other.width ||
             height != other.height;
    }
  };

  Tag tag;
  union {
    StyleExplicitSize_Body explicit_size;
  };

  static StyleGenericBackgroundSize ExplicitSize(const StyleGenericLengthPercentageOrAuto<LengthPercent> &width,
                                                 const StyleGenericLengthPercentageOrAuto<LengthPercent> &height) {
    StyleGenericBackgroundSize result;
    ::new (&result.explicit_size.width) (StyleGenericLengthPercentageOrAuto<LengthPercent>)(width);
    ::new (&result.explicit_size.height) (StyleGenericLengthPercentageOrAuto<LengthPercent>)(height);
    result.tag = Tag::ExplicitSize;
    return result;
  }

  bool IsExplicitSize() const {
    return tag == Tag::ExplicitSize;
  }

  const StyleExplicitSize_Body& AsExplicitSize() const {
    MOZ_DIAGNOSTIC_ASSERT(IsExplicitSize());
    return explicit_size;
  }

  static StyleGenericBackgroundSize Cover() {
    StyleGenericBackgroundSize result;
    result.tag = Tag::Cover;
    return result;
  }

  bool IsCover() const {
    return tag == Tag::Cover;
  }

  static StyleGenericBackgroundSize Contain() {
    StyleGenericBackgroundSize result;
    result.tag = Tag::Contain;
    return result;
  }

  bool IsContain() const {
    return tag == Tag::Contain;
  }

  bool operator==(const StyleGenericBackgroundSize& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::ExplicitSize: return explicit_size == other.explicit_size;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericBackgroundSize& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericBackgroundSize() {

  }
  public:


  ~StyleGenericBackgroundSize() {
    switch (tag) {
      case Tag::ExplicitSize: explicit_size.~StyleExplicitSize_Body(); break;
      default: break;
    }
  }

  StyleGenericBackgroundSize(const StyleGenericBackgroundSize& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::ExplicitSize: ::new (&explicit_size) (StyleExplicitSize_Body)(other.explicit_size); break;
      default: break;
    }
  }
  StyleGenericBackgroundSize& operator=(const StyleGenericBackgroundSize& other) {
    if (this != &other) {
      this->~StyleGenericBackgroundSize();
      new (this) StyleGenericBackgroundSize(other);
    }
    return *this;
  }
  bool IsInitialValue() const;
};

/// A computed value for the `background-size` property.
using StyleBackgroundSize = StyleGenericBackgroundSize<StyleNonNegativeLengthPercentage>;

struct StyleNumberOrPercentage {
  enum class Tag : uint8_t {
    Percentage,
    Number,
  };

  struct StylePercentage_Body {
    StylePercentage _0;

    bool operator==(const StylePercentage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StylePercentage_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleNumber_Body {
    StyleNumber _0;

    bool operator==(const StyleNumber_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleNumber_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StylePercentage_Body percentage;
    StyleNumber_Body number;
  };

  static StyleNumberOrPercentage Percentage(const StylePercentage &_0) {
    StyleNumberOrPercentage result;
    ::new (&result.percentage._0) (StylePercentage)(_0);
    result.tag = Tag::Percentage;
    return result;
  }

  bool IsPercentage() const {
    return tag == Tag::Percentage;
  }

  const StylePercentage& AsPercentage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPercentage());
    return percentage._0;
  }

  static StyleNumberOrPercentage Number(const StyleNumber &_0) {
    StyleNumberOrPercentage result;
    ::new (&result.number._0) (StyleNumber)(_0);
    result.tag = Tag::Number;
    return result;
  }

  bool IsNumber() const {
    return tag == Tag::Number;
  }

  const StyleNumber& AsNumber() const {
    MOZ_DIAGNOSTIC_ASSERT(IsNumber());
    return number._0;
  }

  bool operator==(const StyleNumberOrPercentage& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Percentage: return percentage == other.percentage;
      case Tag::Number: return number == other.number;

    }
    return true;
  }

  bool operator!=(const StyleNumberOrPercentage& other) const {
    return !(*this == other);
  }

  private:
  StyleNumberOrPercentage() {

  }
  public:


  ~StyleNumberOrPercentage() {
    switch (tag) {
      case Tag::Percentage: percentage.~StylePercentage_Body(); break;
      case Tag::Number: number.~StyleNumber_Body(); break;

    }
  }

  StyleNumberOrPercentage(const StyleNumberOrPercentage& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Percentage: ::new (&percentage) (StylePercentage_Body)(other.percentage); break;
      case Tag::Number: ::new (&number) (StyleNumber_Body)(other.number); break;

    }
  }
  StyleNumberOrPercentage& operator=(const StyleNumberOrPercentage& other) {
    if (this != &other) {
      this->~StyleNumberOrPercentage();
      new (this) StyleNumberOrPercentage(other);
    }
    return *this;
  }
};

/// A non-negative <number-percentage>.
using StyleNonNegativeNumberOrPercentage = StyleNonNegative<StyleNumberOrPercentage>;

/// A generic value for the `border-image-slice` property.
template<typename NumberOrPercentage>
struct StyleGenericBorderImageSlice {
  /// The offsets.
  StyleRect<NumberOrPercentage> offsets;
  /// Whether to fill the middle part.
  bool fill;

  bool operator==(const StyleGenericBorderImageSlice& other) const {
    return offsets == other.offsets &&
           fill == other.fill;
  }
  bool operator!=(const StyleGenericBorderImageSlice& other) const {
    return offsets != other.offsets ||
           fill != other.fill;
  }
};

/// A computed value for the `border-image-slice` property.
using StyleBorderImageSlice = StyleGenericBorderImageSlice<StyleNonNegativeNumberOrPercentage>;

/// A generic value for the `border-spacing` property.
template<typename L>
using StyleBorderSpacing = StyleSize2D<L>;

/// A computed value for the `border-radius` property.
using StyleBorderRadius = StyleGenericBorderRadius<StyleNonNegativeLengthPercentage>;

/// A generic `<length>` | `<number>` value for the `-moz-tab-size` property.
template<typename L, typename N>
struct StyleGenericLengthOrNumber {
  enum class Tag : uint8_t {
    /// A number.
    ///
    /// NOTE: Numbers need to be before lengths, in order to parse them
    /// first, since `0` should be a number, not the `0px` length.
    Number,
    /// A length.
    Length,
  };

  struct StyleNumber_Body {
    N _0;

    bool operator==(const StyleNumber_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleNumber_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleLength_Body {
    L _0;

    bool operator==(const StyleLength_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLength_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleNumber_Body number;
    StyleLength_Body length;
  };

  static StyleGenericLengthOrNumber Number(const N &_0) {
    StyleGenericLengthOrNumber result;
    ::new (&result.number._0) (N)(_0);
    result.tag = Tag::Number;
    return result;
  }

  bool IsNumber() const {
    return tag == Tag::Number;
  }

  const N& AsNumber() const {
    MOZ_DIAGNOSTIC_ASSERT(IsNumber());
    return number._0;
  }

  static StyleGenericLengthOrNumber Length(const L &_0) {
    StyleGenericLengthOrNumber result;
    ::new (&result.length._0) (L)(_0);
    result.tag = Tag::Length;
    return result;
  }

  bool IsLength() const {
    return tag == Tag::Length;
  }

  const L& AsLength() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLength());
    return length._0;
  }

  bool operator==(const StyleGenericLengthOrNumber& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Number: return number == other.number;
      case Tag::Length: return length == other.length;

    }
    return true;
  }

  bool operator!=(const StyleGenericLengthOrNumber& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericLengthOrNumber() {

  }
  public:


  ~StyleGenericLengthOrNumber() {
    switch (tag) {
      case Tag::Number: number.~StyleNumber_Body(); break;
      case Tag::Length: length.~StyleLength_Body(); break;

    }
  }

  StyleGenericLengthOrNumber(const StyleGenericLengthOrNumber& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Number: ::new (&number) (StyleNumber_Body)(other.number); break;
      case Tag::Length: ::new (&length) (StyleLength_Body)(other.length); break;

    }
  }
  StyleGenericLengthOrNumber& operator=(const StyleGenericLengthOrNumber& other) {
    if (this != &other) {
      this->~StyleGenericLengthOrNumber();
      new (this) StyleGenericLengthOrNumber(other);
    }
    return *this;
  }
};

/// Either a non-negative `<length>` or a `<number>`.
using StyleNonNegativeLengthOrNumber = StyleGenericLengthOrNumber<StyleNonNegativeLength, StyleNonNegativeNumber>;

/// A specified rectangle made of four `<length-or-number>` values.
using StyleNonNegativeLengthOrNumberRect = StyleRect<StyleNonNegativeLengthOrNumber>;

/// A generic value for the `perspective` property.
template<typename NonNegativeLength>
struct StyleGenericPerspective {
  enum class Tag : uint8_t {
    /// A non-negative length.
    Length,
    /// The keyword `none`.
    None,
  };

  struct StyleLength_Body {
    NonNegativeLength _0;

    bool operator==(const StyleLength_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLength_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleLength_Body length;
  };

  static StyleGenericPerspective Length(const NonNegativeLength &_0) {
    StyleGenericPerspective result;
    ::new (&result.length._0) (NonNegativeLength)(_0);
    result.tag = Tag::Length;
    return result;
  }

  bool IsLength() const {
    return tag == Tag::Length;
  }

  const NonNegativeLength& AsLength() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLength());
    return length._0;
  }

  static StyleGenericPerspective None() {
    StyleGenericPerspective result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  bool operator==(const StyleGenericPerspective& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Length: return length == other.length;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericPerspective& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericPerspective() {

  }
  public:


  ~StyleGenericPerspective() {
    switch (tag) {
      case Tag::Length: length.~StyleLength_Body(); break;
      default: break;
    }
  }

  StyleGenericPerspective(const StyleGenericPerspective& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Length: ::new (&length) (StyleLength_Body)(other.length); break;
      default: break;
    }
  }
  StyleGenericPerspective& operator=(const StyleGenericPerspective& other) {
    if (this != &other) {
      this->~StyleGenericPerspective();
      new (this) StyleGenericPerspective(other);
    }
    return *this;
  }
};

/// A computed value for the `perspective` property.
using StylePerspective = StyleGenericPerspective<StyleNonNegativeLength>;

/// A generic value for the `z-index` property.
template<typename I>
struct StyleGenericZIndex {
  enum class Tag : uint8_t {
    /// An integer value.
    Integer,
    /// The keyword `auto`.
    Auto,
  };

  struct StyleInteger_Body {
    I _0;

    bool operator==(const StyleInteger_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleInteger_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleInteger_Body integer;
  };

  static StyleGenericZIndex Integer(const I &_0) {
    StyleGenericZIndex result;
    ::new (&result.integer._0) (I)(_0);
    result.tag = Tag::Integer;
    return result;
  }

  bool IsInteger() const {
    return tag == Tag::Integer;
  }

  const I& AsInteger() const {
    MOZ_DIAGNOSTIC_ASSERT(IsInteger());
    return integer._0;
  }

  static StyleGenericZIndex Auto() {
    StyleGenericZIndex result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  bool operator==(const StyleGenericZIndex& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Integer: return integer == other.integer;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericZIndex& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericZIndex() {

  }
  public:


  ~StyleGenericZIndex() {
    switch (tag) {
      case Tag::Integer: integer.~StyleInteger_Body(); break;
      default: break;
    }
  }

  StyleGenericZIndex(const StyleGenericZIndex& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Integer: ::new (&integer) (StyleInteger_Body)(other.integer); break;
      default: break;
    }
  }
  StyleGenericZIndex& operator=(const StyleGenericZIndex& other) {
    if (this != &other) {
      this->~StyleGenericZIndex();
      new (this) StyleGenericZIndex(other);
    }
    return *this;
  }
};

/// A computed value for the `z-index` property.
using StyleZIndex = StyleGenericZIndex<StyleInteger>;

/// A generic transform origin.
template<typename H, typename V, typename Depth>
struct StyleGenericTransformOrigin {
  /// The horizontal origin.
  H horizontal;
  /// The vertical origin.
  V vertical;
  /// The depth.
  Depth depth;

  bool operator==(const StyleGenericTransformOrigin& other) const {
    return horizontal == other.horizontal &&
           vertical == other.vertical &&
           depth == other.depth;
  }
  bool operator!=(const StyleGenericTransformOrigin& other) const {
    return horizontal != other.horizontal ||
           vertical != other.vertical ||
           depth != other.depth;
  }
  inline bool HasPercent() const;
};

/// The computed value of a CSS `<transform-origin>`
using StyleTransformOrigin = StyleGenericTransformOrigin<StyleLengthPercentage, StyleLengthPercentage, StyleLength>;

/// Constants for contain: https://drafts.csswg.org/css-contain/#contain-property
struct StyleContain {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleContain operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleContain operator|(const StyleContain& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleContain& operator|=(const StyleContain& other) {
    *this = (*this | other);
    return *this;
  }
  StyleContain operator&(const StyleContain& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleContain& operator&=(const StyleContain& other) {
    *this = (*this & other);
    return *this;
  }
  StyleContain operator^(const StyleContain& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleContain& operator^=(const StyleContain& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleContain& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleContain& other) const {
    return bits != other.bits;
  }
  static const StyleContain NONE;
  static const StyleContain SIZE;
  static const StyleContain LAYOUT;
  static const StyleContain PAINT;
  static const StyleContain STRICT;
  static const StyleContain CONTENT;
};
/// `none` variant, just for convenience.
inline const StyleContain StyleContain::NONE = StyleContain{ /* .bits = */ (uint8_t)0 };
/// 'size' variant, turns on size containment
inline const StyleContain StyleContain::SIZE = StyleContain{ /* .bits = */ (uint8_t)(1 << 0) };
/// `layout` variant, turns on layout containment
inline const StyleContain StyleContain::LAYOUT = StyleContain{ /* .bits = */ (uint8_t)(1 << 1) };
/// `paint` variant, turns on paint containment
inline const StyleContain StyleContain::PAINT = StyleContain{ /* .bits = */ (uint8_t)(1 << 2) };
/// `strict` variant, turns on all types of containment
inline const StyleContain StyleContain::STRICT = StyleContain{ /* .bits = */ (uint8_t)(1 << 3) };
/// 'content' variant, turns on layout and paint containment
inline const StyleContain StyleContain::CONTENT = StyleContain{ /* .bits = */ (uint8_t)(1 << 4) };

/// Values for the `touch-action` property.
/// These constants match Gecko's `NS_STYLE_TOUCH_ACTION_*` constants.
struct StyleTouchAction {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleTouchAction operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleTouchAction operator|(const StyleTouchAction& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleTouchAction& operator|=(const StyleTouchAction& other) {
    *this = (*this | other);
    return *this;
  }
  StyleTouchAction operator&(const StyleTouchAction& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleTouchAction& operator&=(const StyleTouchAction& other) {
    *this = (*this & other);
    return *this;
  }
  StyleTouchAction operator^(const StyleTouchAction& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleTouchAction& operator^=(const StyleTouchAction& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleTouchAction& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleTouchAction& other) const {
    return bits != other.bits;
  }
  static const StyleTouchAction NONE;
  static const StyleTouchAction AUTO;
  static const StyleTouchAction PAN_X;
  static const StyleTouchAction PAN_Y;
  static const StyleTouchAction MANIPULATION;
  static const StyleTouchAction PINCH_ZOOM;
};
/// `none` variant
inline const StyleTouchAction StyleTouchAction::NONE = StyleTouchAction{ /* .bits = */ (uint8_t)(1 << 0) };
/// `auto` variant
inline const StyleTouchAction StyleTouchAction::AUTO = StyleTouchAction{ /* .bits = */ (uint8_t)(1 << 1) };
/// `pan-x` variant
inline const StyleTouchAction StyleTouchAction::PAN_X = StyleTouchAction{ /* .bits = */ (uint8_t)(1 << 2) };
/// `pan-y` variant
inline const StyleTouchAction StyleTouchAction::PAN_Y = StyleTouchAction{ /* .bits = */ (uint8_t)(1 << 3) };
/// `manipulation` variant
inline const StyleTouchAction StyleTouchAction::MANIPULATION = StyleTouchAction{ /* .bits = */ (uint8_t)(1 << 4) };
/// `pinch-zoom` variant
inline const StyleTouchAction StyleTouchAction::PINCH_ZOOM = StyleTouchAction{ /* .bits = */ (uint8_t)(1 << 5) };

#if defined(CBINDGEN_IS_GECKO)
/// A handle to a Gecko atom. This is a type that can represent either:
///
///  * A strong reference to a dynamic atom (an `nsAtom` pointer), in which case
///    the `usize` just holds the pointer value.
///
///  * A byte offset from `gGkAtoms` to the `nsStaticAtom` object (shifted to
///    the left one bit, and with the lower bit set to `1` to differentiate it
///    from the above), so `(offset << 1 | 1)`.
///
struct StyleAtom {
  StyleNonZeroUsize _0;

  bool operator==(const StyleAtom& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleAtom& other) const {
    return _0 != other._0;
  }
  StyleAtom(size_t) = delete;
  StyleAtom() = delete;

  inline bool IsStatic() const;
  inline nsAtom* AsAtom() const;

 private:
  inline void AddRef();
  inline void Release();

 public:
  inline explicit StyleAtom(already_AddRefed<nsAtom> aAtom);
  inline StyleAtom(const StyleAtom& aOther);
  inline StyleAtom& operator=(const StyleAtom&);
  inline ~StyleAtom();
};
#endif

/// <https://drafts.csswg.org/css-values-4/#custom-idents>
struct StyleCustomIdent {
  StyleAtom _0;

  bool operator==(const StyleCustomIdent& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleCustomIdent& other) const {
    return _0 != other._0;
  }
  inline nsAtom* AsAtom() const;
};

/// The change bits that we care about.
struct StyleWillChangeBits {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleWillChangeBits operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleWillChangeBits operator|(const StyleWillChangeBits& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleWillChangeBits& operator|=(const StyleWillChangeBits& other) {
    *this = (*this | other);
    return *this;
  }
  StyleWillChangeBits operator&(const StyleWillChangeBits& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleWillChangeBits& operator&=(const StyleWillChangeBits& other) {
    *this = (*this & other);
    return *this;
  }
  StyleWillChangeBits operator^(const StyleWillChangeBits& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleWillChangeBits& operator^=(const StyleWillChangeBits& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleWillChangeBits& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleWillChangeBits& other) const {
    return bits != other.bits;
  }
  static const StyleWillChangeBits STACKING_CONTEXT;
  static const StyleWillChangeBits TRANSFORM;
  static const StyleWillChangeBits SCROLL;
  static const StyleWillChangeBits OPACITY;
  static const StyleWillChangeBits FIXPOS_CB;
  static const StyleWillChangeBits ABSPOS_CB;
};
/// Whether the stacking context will change.
inline const StyleWillChangeBits StyleWillChangeBits::STACKING_CONTEXT = StyleWillChangeBits{ /* .bits = */ (uint8_t)(1 << 0) };
/// Whether `transform` will change.
inline const StyleWillChangeBits StyleWillChangeBits::TRANSFORM = StyleWillChangeBits{ /* .bits = */ (uint8_t)(1 << 1) };
/// Whether `scroll-position` will change.
inline const StyleWillChangeBits StyleWillChangeBits::SCROLL = StyleWillChangeBits{ /* .bits = */ (uint8_t)(1 << 2) };
/// Whether `opacity` will change.
inline const StyleWillChangeBits StyleWillChangeBits::OPACITY = StyleWillChangeBits{ /* .bits = */ (uint8_t)(1 << 3) };
/// Fixed pos containing block.
inline const StyleWillChangeBits StyleWillChangeBits::FIXPOS_CB = StyleWillChangeBits{ /* .bits = */ (uint8_t)(1 << 4) };
/// Abs pos containing block.
inline const StyleWillChangeBits StyleWillChangeBits::ABSPOS_CB = StyleWillChangeBits{ /* .bits = */ (uint8_t)(1 << 5) };

/// Provides a rendering hint to the user agent, stating what kinds of changes
/// the author expects to perform on the element.
///
/// `auto` is represented by an empty `features` list.
///
/// <https://drafts.csswg.org/css-will-change/#will-change>
struct StyleWillChange {
  /// The features that are supposed to change.
  ///
  /// TODO(emilio): Consider using ArcSlice since we just clone them from the
  /// specified value? That'd save an allocation, which could be worth it.
  StyleOwnedSlice<StyleCustomIdent> features;
  /// A bitfield with the kind of change that the value will create, based
  /// on the above field.
  StyleWillChangeBits bits;

  bool operator==(const StyleWillChange& other) const {
    return features == other.features &&
           bits == other.bits;
  }
  bool operator!=(const StyleWillChange& other) const {
    return features != other.features ||
           bits != other.bits;
  }
};

/// Specified keyword values for the text-decoration-line property.
struct StyleTextDecorationLine {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleTextDecorationLine operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleTextDecorationLine operator|(const StyleTextDecorationLine& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleTextDecorationLine& operator|=(const StyleTextDecorationLine& other) {
    *this = (*this | other);
    return *this;
  }
  StyleTextDecorationLine operator&(const StyleTextDecorationLine& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleTextDecorationLine& operator&=(const StyleTextDecorationLine& other) {
    *this = (*this & other);
    return *this;
  }
  StyleTextDecorationLine operator^(const StyleTextDecorationLine& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleTextDecorationLine& operator^=(const StyleTextDecorationLine& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleTextDecorationLine& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleTextDecorationLine& other) const {
    return bits != other.bits;
  }
  static const StyleTextDecorationLine NONE;
  static const StyleTextDecorationLine UNDERLINE;
  static const StyleTextDecorationLine OVERLINE;
  static const StyleTextDecorationLine LINE_THROUGH;
  static const StyleTextDecorationLine BLINK;
  static const StyleTextDecorationLine COLOR_OVERRIDE;
};
/// No text decoration line is specified.
inline const StyleTextDecorationLine StyleTextDecorationLine::NONE = StyleTextDecorationLine{ /* .bits = */ (uint8_t)0 };
/// underline
inline const StyleTextDecorationLine StyleTextDecorationLine::UNDERLINE = StyleTextDecorationLine{ /* .bits = */ (uint8_t)(1 << 0) };
/// overline
inline const StyleTextDecorationLine StyleTextDecorationLine::OVERLINE = StyleTextDecorationLine{ /* .bits = */ (uint8_t)(1 << 1) };
/// line-through
inline const StyleTextDecorationLine StyleTextDecorationLine::LINE_THROUGH = StyleTextDecorationLine{ /* .bits = */ (uint8_t)(1 << 2) };
/// blink
inline const StyleTextDecorationLine StyleTextDecorationLine::BLINK = StyleTextDecorationLine{ /* .bits = */ (uint8_t)(1 << 3) };
#if defined(CBINDGEN_IS_GECKO)
/// Only set by presentation attributes
///
/// Setting this will mean that text-decorations use the color
/// specified by `color` in quirks mode.
///
/// For example, this gives <a href=foo><font color="red">text</font></a>
/// a red text decoration
inline const StyleTextDecorationLine StyleTextDecorationLine::COLOR_OVERRIDE = StyleTextDecorationLine{ /* .bits = */ (uint8_t)16 };
#endif

/// Specified keyword values for the text-underline-position property.
/// (Non-exclusive, but not all combinations are allowed: the spec grammar gives
/// `auto | [ from-font | under ] || [ left | right ]`.)
/// https://drafts.csswg.org/css-text-decor-4/#text-underline-position-property
struct StyleTextUnderlinePosition {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleTextUnderlinePosition operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleTextUnderlinePosition operator|(const StyleTextUnderlinePosition& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleTextUnderlinePosition& operator|=(const StyleTextUnderlinePosition& other) {
    *this = (*this | other);
    return *this;
  }
  StyleTextUnderlinePosition operator&(const StyleTextUnderlinePosition& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleTextUnderlinePosition& operator&=(const StyleTextUnderlinePosition& other) {
    *this = (*this & other);
    return *this;
  }
  StyleTextUnderlinePosition operator^(const StyleTextUnderlinePosition& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleTextUnderlinePosition& operator^=(const StyleTextUnderlinePosition& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleTextUnderlinePosition& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleTextUnderlinePosition& other) const {
    return bits != other.bits;
  }
  inline bool IsAuto() const;
  inline bool IsFromFont() const;
  inline bool IsUnder() const;
  inline bool IsLeft() const;
  inline bool IsRight() const;
  static const StyleTextUnderlinePosition AUTO;
  static const StyleTextUnderlinePosition FROM_FONT;
  static const StyleTextUnderlinePosition UNDER;
  static const StyleTextUnderlinePosition LEFT;
  static const StyleTextUnderlinePosition RIGHT;
};
/// Use automatic positioning below the alphabetic baseline.
inline const StyleTextUnderlinePosition StyleTextUnderlinePosition::AUTO = StyleTextUnderlinePosition{ /* .bits = */ (uint8_t)0 };
/// Use underline position from the first available font.
inline const StyleTextUnderlinePosition StyleTextUnderlinePosition::FROM_FONT = StyleTextUnderlinePosition{ /* .bits = */ (uint8_t)(1 << 0) };
/// Below the glyph box.
inline const StyleTextUnderlinePosition StyleTextUnderlinePosition::UNDER = StyleTextUnderlinePosition{ /* .bits = */ (uint8_t)(1 << 1) };
/// In vertical mode, place to the left of the text.
inline const StyleTextUnderlinePosition StyleTextUnderlinePosition::LEFT = StyleTextUnderlinePosition{ /* .bits = */ (uint8_t)(1 << 2) };
/// In vertical mode, place to the right of the text.
inline const StyleTextUnderlinePosition StyleTextUnderlinePosition::RIGHT = StyleTextUnderlinePosition{ /* .bits = */ (uint8_t)(1 << 3) };

/// Specified keyword values for non-case transforms in the text-transform property. (Non-exclusive.)
struct StyleTextTransformOther {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleTextTransformOther operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleTextTransformOther operator|(const StyleTextTransformOther& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleTextTransformOther& operator|=(const StyleTextTransformOther& other) {
    *this = (*this | other);
    return *this;
  }
  StyleTextTransformOther operator&(const StyleTextTransformOther& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleTextTransformOther& operator&=(const StyleTextTransformOther& other) {
    *this = (*this & other);
    return *this;
  }
  StyleTextTransformOther operator^(const StyleTextTransformOther& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleTextTransformOther& operator^=(const StyleTextTransformOther& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleTextTransformOther& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleTextTransformOther& other) const {
    return bits != other.bits;
  }
  static const StyleTextTransformOther FULL_WIDTH;
  static const StyleTextTransformOther FULL_SIZE_KANA;
};
/// full-width
inline const StyleTextTransformOther StyleTextTransformOther::FULL_WIDTH = StyleTextTransformOther{ /* .bits = */ (uint8_t)(1 << 0) };
/// full-size-kana
inline const StyleTextTransformOther StyleTextTransformOther::FULL_SIZE_KANA = StyleTextTransformOther{ /* .bits = */ (uint8_t)(1 << 1) };

/// Specified value of the text-transform property, stored in two parts:
/// the case-related transforms (mutually exclusive, only one may be in effect), and others (non-exclusive).
struct StyleTextTransform {
  /// Case transform, if any.
  StyleTextTransformCase case_;
  /// Non-case transforms.
  StyleTextTransformOther other_;

  bool operator==(const StyleTextTransform& other) const {
    return case_ == other.case_ &&
           other_ == other.other_;
  }
  bool operator!=(const StyleTextTransform& other) const {
    return case_ != other.case_ ||
           other_ != other.other_;
  }
  static inline StyleTextTransform None();
  inline bool IsNone() const;
};

/// A generic value for the `text-overflow` property.
struct StyleTextOverflowSide {
  enum class Tag : uint8_t {
    /// Clip inline content.
    Clip,
    /// Render ellipsis to represent clipped inline content.
    Ellipsis,
    /// Render a given string to represent clipped inline content.
    String,
  };

  struct StyleString_Body {
    StyleOwnedStr _0;

    bool operator==(const StyleString_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleString_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleString_Body string;
  };

  static StyleTextOverflowSide Clip() {
    StyleTextOverflowSide result;
    result.tag = Tag::Clip;
    return result;
  }

  bool IsClip() const {
    return tag == Tag::Clip;
  }

  static StyleTextOverflowSide Ellipsis() {
    StyleTextOverflowSide result;
    result.tag = Tag::Ellipsis;
    return result;
  }

  bool IsEllipsis() const {
    return tag == Tag::Ellipsis;
  }

  static StyleTextOverflowSide String(const StyleOwnedStr &_0) {
    StyleTextOverflowSide result;
    ::new (&result.string._0) (StyleOwnedStr)(_0);
    result.tag = Tag::String;
    return result;
  }

  bool IsString() const {
    return tag == Tag::String;
  }

  const StyleOwnedStr& AsString() const {
    MOZ_DIAGNOSTIC_ASSERT(IsString());
    return string._0;
  }

  bool operator==(const StyleTextOverflowSide& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::String: return string == other.string;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleTextOverflowSide& other) const {
    return !(*this == other);
  }

  private:
  StyleTextOverflowSide() {

  }
  public:


  ~StyleTextOverflowSide() {
    switch (tag) {
      case Tag::String: string.~StyleString_Body(); break;
      default: break;
    }
  }

  StyleTextOverflowSide(const StyleTextOverflowSide& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::String: ::new (&string) (StyleString_Body)(other.string); break;
      default: break;
    }
  }
  StyleTextOverflowSide& operator=(const StyleTextOverflowSide& other) {
    if (this != &other) {
      this->~StyleTextOverflowSide();
      new (this) StyleTextOverflowSide(other);
    }
    return *this;
  }
};

/// text-overflow.
/// When the specified value only has one side, that's the "second"
/// side, and the sides are logical, so "second" means "end".  The
/// start side is Clip in that case.
///
/// When the specified value has two sides, those are our "first"
/// and "second" sides, and they are physical sides ("left" and
/// "right").
struct StyleTextOverflow {
  /// First side
  StyleTextOverflowSide first;
  /// Second side
  StyleTextOverflowSide second;
  /// True if the specified value only has one side.
  bool sides_are_logical;

  bool operator==(const StyleTextOverflow& other) const {
    return first == other.first &&
           second == other.second &&
           sides_are_logical == other.sides_are_logical;
  }
  bool operator!=(const StyleTextOverflow& other) const {
    return first != other.first ||
           second != other.second ||
           sides_are_logical != other.sides_are_logical;
  }
  StyleTextOverflow()
    : first(StyleTextOverflowSide::Clip()),
      second(StyleTextOverflowSide::Clip()),
      sides_are_logical(true) {}
};

/// A color with red, green, blue, and alpha components, in a byte each.
struct StyleRGBA {
  /// The red component.
  uint8_t red;
  /// The green component.
  uint8_t green;
  /// The blue component.
  uint8_t blue;
  /// The alpha component.
  uint8_t alpha;

  bool operator==(const StyleRGBA& other) const {
    return red == other.red &&
           green == other.green &&
           blue == other.blue &&
           alpha == other.alpha;
  }
  bool operator!=(const StyleRGBA& other) const {
    return red != other.red ||
           green != other.green ||
           blue != other.blue ||
           alpha != other.alpha;
  }
  static inline StyleRGBA Transparent();
  static inline StyleRGBA FromColor(nscolor);

  inline nscolor ToColor() const;
};

/// Ratios representing the contribution of color and currentcolor to
/// the final color value.
struct StyleComplexColorRatios {
  /// Numeric color contribution.
  float bg;
  /// currentcolor contribution.
  float fg;

  bool operator==(const StyleComplexColorRatios& other) const {
    return bg == other.bg &&
           fg == other.fg;
  }
  bool operator!=(const StyleComplexColorRatios& other) const {
    return bg != other.bg ||
           fg != other.fg;
  }
  static const StyleComplexColorRatios NUMERIC;
  static const StyleComplexColorRatios CURRENT_COLOR;
};
/// Ratios representing a `Numeric` color.
inline const StyleComplexColorRatios StyleComplexColorRatios::NUMERIC = StyleComplexColorRatios{ /* .bg = */ 1., /* .fg = */ 0. };
/// Ratios representing the `CurrentColor` color.
inline const StyleComplexColorRatios StyleComplexColorRatios::CURRENT_COLOR = StyleComplexColorRatios{ /* .bg = */ 0., /* .fg = */ 1. };

/// This enum represents a combined color from a numeric color and
/// the current foreground color (currentcolor keyword).
template<typename RGBA>
struct StyleGenericColor {
  enum class Tag : uint8_t {
    ///  Numeric RGBA color.
    Numeric,
    /// The current foreground color.
    CurrentColor,
    /// A linear combination of numeric color and currentcolor.
    /// The formula is: `color * ratios.bg + currentcolor * ratios.fg`.
    Complex,
  };

  struct StyleNumeric_Body {
    RGBA _0;

    bool operator==(const StyleNumeric_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleNumeric_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleComplex_Body {
    /// The actual numeric color.
    RGBA color;
    /// The ratios of mixing between numeric and currentcolor.
    StyleComplexColorRatios ratios;

    bool operator==(const StyleComplex_Body& other) const {
      return color == other.color &&
             ratios == other.ratios;
    }
    bool operator!=(const StyleComplex_Body& other) const {
      return color != other.color ||
             ratios != other.ratios;
    }
  };

  Tag tag;
  union {
    StyleNumeric_Body numeric;
    StyleComplex_Body complex;
  };

  static StyleGenericColor Numeric(const RGBA &_0) {
    StyleGenericColor result;
    ::new (&result.numeric._0) (RGBA)(_0);
    result.tag = Tag::Numeric;
    return result;
  }

  bool IsNumeric() const {
    return tag == Tag::Numeric;
  }

  const RGBA& AsNumeric() const {
    MOZ_DIAGNOSTIC_ASSERT(IsNumeric());
    return numeric._0;
  }

  static StyleGenericColor CurrentColor() {
    StyleGenericColor result;
    result.tag = Tag::CurrentColor;
    return result;
  }

  bool IsCurrentColor() const {
    return tag == Tag::CurrentColor;
  }

  static StyleGenericColor Complex(const RGBA &color,
                                   const StyleComplexColorRatios &ratios) {
    StyleGenericColor result;
    ::new (&result.complex.color) (RGBA)(color);
    ::new (&result.complex.ratios) (StyleComplexColorRatios)(ratios);
    result.tag = Tag::Complex;
    return result;
  }

  bool IsComplex() const {
    return tag == Tag::Complex;
  }

  const StyleComplex_Body& AsComplex() const {
    MOZ_DIAGNOSTIC_ASSERT(IsComplex());
    return complex;
  }

  bool operator==(const StyleGenericColor& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Numeric: return numeric == other.numeric;
      case Tag::Complex: return complex == other.complex;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericColor& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericColor() {

  }
  public:


  ~StyleGenericColor() {
    switch (tag) {
      case Tag::Numeric: numeric.~StyleNumeric_Body(); break;
      case Tag::Complex: complex.~StyleComplex_Body(); break;
      default: break;
    }
  }

  StyleGenericColor(const StyleGenericColor& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Numeric: ::new (&numeric) (StyleNumeric_Body)(other.numeric); break;
      case Tag::Complex: ::new (&complex) (StyleComplex_Body)(other.complex); break;
      default: break;
    }
  }
  StyleGenericColor& operator=(const StyleGenericColor& other) {
    if (this != &other) {
      this->~StyleGenericColor();
      new (this) StyleGenericColor(other);
    }
    return *this;
  }
  static inline StyleGenericColor FromColor(nscolor);
  static inline StyleGenericColor Black();
  static inline StyleGenericColor White();
  static inline StyleGenericColor Transparent();
  bool MaybeTransparent() const;
  /**
   * Compute the final color, taking into account the foreground color from the
   * frame's ComputedStyle.
   */
  nscolor CalcColor(const nsIFrame*) const;
  /**
   * Compute the final color, taking into account the foreground color from the
   * style.
   */
  nscolor CalcColor(const ComputedStyle&) const;
  /**
   * Compute the final color, making the argument the foreground color.
   */
  nscolor CalcColor(nscolor) const;
  nscolor CalcColor(const StyleRGBA&) const;
};

/// An animated value for `<color>`.
using StyleColor = StyleGenericColor<StyleRGBA>;

/// A generic value for `scrollbar-color` property.
///
/// https://drafts.csswg.org/css-scrollbars-1/#scrollbar-color
template<typename Color>
struct StyleGenericScrollbarColor {
  enum class Tag : uint8_t {
    /// `auto`
    Auto,
    /// `<color>{2}`
    Colors,
  };

  struct StyleColors_Body {
    /// First `<color>`, for color of the scrollbar thumb.
    Color thumb;
    /// Second `<color>`, for color of the scrollbar track.
    Color track;

    bool operator==(const StyleColors_Body& other) const {
      return thumb == other.thumb &&
             track == other.track;
    }
    bool operator!=(const StyleColors_Body& other) const {
      return thumb != other.thumb ||
             track != other.track;
    }
  };

  Tag tag;
  union {
    StyleColors_Body colors;
  };

  static StyleGenericScrollbarColor Auto() {
    StyleGenericScrollbarColor result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  static StyleGenericScrollbarColor Colors(const Color &thumb,
                                           const Color &track) {
    StyleGenericScrollbarColor result;
    ::new (&result.colors.thumb) (Color)(thumb);
    ::new (&result.colors.track) (Color)(track);
    result.tag = Tag::Colors;
    return result;
  }

  bool IsColors() const {
    return tag == Tag::Colors;
  }

  const StyleColors_Body& AsColors() const {
    MOZ_DIAGNOSTIC_ASSERT(IsColors());
    return colors;
  }

  bool operator==(const StyleGenericScrollbarColor& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Colors: return colors == other.colors;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericScrollbarColor& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericScrollbarColor() {

  }
  public:


  ~StyleGenericScrollbarColor() {
    switch (tag) {
      case Tag::Colors: colors.~StyleColors_Body(); break;
      default: break;
    }
  }

  StyleGenericScrollbarColor(const StyleGenericScrollbarColor& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Colors: ::new (&colors) (StyleColors_Body)(other.colors); break;
      default: break;
    }
  }
  StyleGenericScrollbarColor& operator=(const StyleGenericScrollbarColor& other) {
    if (this != &other) {
      this->~StyleGenericScrollbarColor();
      new (this) StyleGenericScrollbarColor(other);
    }
    return *this;
  }
};

/// A computed value for `scrollbar-color` property.
using StyleScrollbarColor = StyleGenericScrollbarColor<StyleColor>;

/// Either `<color>` or `auto`.
template<typename C>
struct StyleGenericColorOrAuto {
  enum class Tag : uint8_t {
    /// A `<color>`.
    Color,
    /// `auto`
    Auto,
  };

  struct StyleColor_Body {
    C _0;

    bool operator==(const StyleColor_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleColor_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleColor_Body color;
  };

  static StyleGenericColorOrAuto Color(const C &_0) {
    StyleGenericColorOrAuto result;
    ::new (&result.color._0) (C)(_0);
    result.tag = Tag::Color;
    return result;
  }

  bool IsColor() const {
    return tag == Tag::Color;
  }

  const C& AsColor() const {
    MOZ_DIAGNOSTIC_ASSERT(IsColor());
    return color._0;
  }

  static StyleGenericColorOrAuto Auto() {
    StyleGenericColorOrAuto result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  bool operator==(const StyleGenericColorOrAuto& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Color: return color == other.color;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericColorOrAuto& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericColorOrAuto() {

  }
  public:


  ~StyleGenericColorOrAuto() {
    switch (tag) {
      case Tag::Color: color.~StyleColor_Body(); break;
      default: break;
    }
  }

  StyleGenericColorOrAuto(const StyleGenericColorOrAuto& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Color: ::new (&color) (StyleColor_Body)(other.color); break;
      default: break;
    }
  }
  StyleGenericColorOrAuto& operator=(const StyleGenericColorOrAuto& other) {
    if (this != &other) {
      this->~StyleGenericColorOrAuto();
      new (this) StyleGenericColorOrAuto(other);
    }
    return *this;
  }
};

/// auto | <color>
using StyleColorOrAuto = StyleGenericColorOrAuto<StyleColor>;

/// A generic value for the `vertical-align` property.
template<typename LengthPercentage>
struct StyleGenericVerticalAlign {
  enum class Tag : uint8_t {
    /// One of the vertical-align keywords.
    Keyword,
    /// `<length-percentage>`
    Length,
  };

  struct StyleKeyword_Body {
    StyleVerticalAlignKeyword _0;

    bool operator==(const StyleKeyword_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleKeyword_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleLength_Body {
    LengthPercentage _0;

    bool operator==(const StyleLength_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLength_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleKeyword_Body keyword;
    StyleLength_Body length;
  };

  static StyleGenericVerticalAlign Keyword(const StyleVerticalAlignKeyword &_0) {
    StyleGenericVerticalAlign result;
    ::new (&result.keyword._0) (StyleVerticalAlignKeyword)(_0);
    result.tag = Tag::Keyword;
    return result;
  }

  bool IsKeyword() const {
    return tag == Tag::Keyword;
  }

  const StyleVerticalAlignKeyword& AsKeyword() const {
    MOZ_DIAGNOSTIC_ASSERT(IsKeyword());
    return keyword._0;
  }

  static StyleGenericVerticalAlign Length(const LengthPercentage &_0) {
    StyleGenericVerticalAlign result;
    ::new (&result.length._0) (LengthPercentage)(_0);
    result.tag = Tag::Length;
    return result;
  }

  bool IsLength() const {
    return tag == Tag::Length;
  }

  const LengthPercentage& AsLength() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLength());
    return length._0;
  }

  bool operator==(const StyleGenericVerticalAlign& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Keyword: return keyword == other.keyword;
      case Tag::Length: return length == other.length;

    }
    return true;
  }

  bool operator!=(const StyleGenericVerticalAlign& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericVerticalAlign() {

  }
  public:


  ~StyleGenericVerticalAlign() {
    switch (tag) {
      case Tag::Keyword: keyword.~StyleKeyword_Body(); break;
      case Tag::Length: length.~StyleLength_Body(); break;

    }
  }

  StyleGenericVerticalAlign(const StyleGenericVerticalAlign& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Keyword: ::new (&keyword) (StyleKeyword_Body)(other.keyword); break;
      case Tag::Length: ::new (&length) (StyleLength_Body)(other.length); break;

    }
  }
  StyleGenericVerticalAlign& operator=(const StyleGenericVerticalAlign& other) {
    if (this != &other) {
      this->~StyleGenericVerticalAlign();
      new (this) StyleGenericVerticalAlign(other);
    }
    return *this;
  }
};

/// A computed value for the `vertical-align` property.
using StyleVerticalAlign = StyleGenericVerticalAlign<StyleLengthPercentage>;

/// The computed value of `ShapeRadius`
using StyleShapeRadius = StyleGenericShapeRadius<StyleNonNegativeLengthPercentage>;

/// The inner pointer of an ArcSlice<T>, to be sent via FFI.
/// The type of the pointer is a bit of a lie, we just want to preserve the type
/// but these pointers cannot be constructed outside of this crate, so we're
/// good.
template<typename T>
struct StyleForgottenArcSlicePtr {
  T *_0;

  bool operator==(const StyleForgottenArcSlicePtr& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleForgottenArcSlicePtr& other) const {
    return _0 != other._0;
  }
};

/// The context properties we understand.
struct StyleContextPropertyBits {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleContextPropertyBits operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleContextPropertyBits operator|(const StyleContextPropertyBits& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleContextPropertyBits& operator|=(const StyleContextPropertyBits& other) {
    *this = (*this | other);
    return *this;
  }
  StyleContextPropertyBits operator&(const StyleContextPropertyBits& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleContextPropertyBits& operator&=(const StyleContextPropertyBits& other) {
    *this = (*this & other);
    return *this;
  }
  StyleContextPropertyBits operator^(const StyleContextPropertyBits& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleContextPropertyBits& operator^=(const StyleContextPropertyBits& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleContextPropertyBits& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleContextPropertyBits& other) const {
    return bits != other.bits;
  }
  static const StyleContextPropertyBits FILL;
  static const StyleContextPropertyBits STROKE;
  static const StyleContextPropertyBits FILL_OPACITY;
  static const StyleContextPropertyBits STROKE_OPACITY;
};
/// `fill`
inline const StyleContextPropertyBits StyleContextPropertyBits::FILL = StyleContextPropertyBits{ /* .bits = */ (uint8_t)(1 << 0) };
/// `stroke`
inline const StyleContextPropertyBits StyleContextPropertyBits::STROKE = StyleContextPropertyBits{ /* .bits = */ (uint8_t)(1 << 1) };
/// `fill-opacity`
inline const StyleContextPropertyBits StyleContextPropertyBits::FILL_OPACITY = StyleContextPropertyBits{ /* .bits = */ (uint8_t)(1 << 2) };
/// `stroke-opacity`
inline const StyleContextPropertyBits StyleContextPropertyBits::STROKE_OPACITY = StyleContextPropertyBits{ /* .bits = */ (uint8_t)(1 << 3) };

/// Specified MozContextProperties value.
/// Nonstandard (https://developer.mozilla.org/en-US/docs/Web/CSS/-moz-context-properties)
struct StyleMozContextProperties {
  StyleArcSlice<StyleCustomIdent> idents;
  StyleContextPropertyBits bits;

  bool operator==(const StyleMozContextProperties& other) const {
    return idents == other.idents &&
           bits == other.bits;
  }
  bool operator!=(const StyleMozContextProperties& other) const {
    return idents != other.idents ||
           bits != other.bits;
  }
};

/// A quote pair.
struct StyleQuotePair {
  /// The opening quote.
  StyleOwnedStr opening;
  /// The closing quote.
  StyleOwnedStr closing;

  bool operator==(const StyleQuotePair& other) const {
    return opening == other.opening &&
           closing == other.closing;
  }
  bool operator!=(const StyleQuotePair& other) const {
    return opening != other.opening ||
           closing != other.closing;
  }
};

/// List of quote pairs for the specified/computed value of `quotes` property.
using StyleQuoteList = StyleArcSlice<StyleQuotePair>;

/// Specified and computed `quotes` property: `auto`, `none`, or a list
/// of characters.
struct StyleQuotes {
  enum class Tag {
    /// list of quote pairs
    QuoteList,
    /// auto (use lang-dependent quote marks)
    Auto,
  };

  struct StyleQuoteList_Body {
    StyleQuoteList _0;

    bool operator==(const StyleQuoteList_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleQuoteList_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleQuoteList_Body quote_list;
  };

  static StyleQuotes QuoteList(const StyleQuoteList &_0) {
    StyleQuotes result;
    ::new (&result.quote_list._0) (StyleQuoteList)(_0);
    result.tag = Tag::QuoteList;
    return result;
  }

  bool IsQuoteList() const {
    return tag == Tag::QuoteList;
  }

  const StyleQuoteList& AsQuoteList() const {
    MOZ_DIAGNOSTIC_ASSERT(IsQuoteList());
    return quote_list._0;
  }

  static StyleQuotes Auto() {
    StyleQuotes result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  bool operator==(const StyleQuotes& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::QuoteList: return quote_list == other.quote_list;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleQuotes& other) const {
    return !(*this == other);
  }

  private:
  StyleQuotes() {

  }
  public:


  ~StyleQuotes() {
    switch (tag) {
      case Tag::QuoteList: quote_list.~StyleQuoteList_Body(); break;
      default: break;
    }
  }

  StyleQuotes(const StyleQuotes& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::QuoteList: ::new (&quote_list) (StyleQuoteList_Body)(other.quote_list); break;
      default: break;
    }
  }
  StyleQuotes& operator=(const StyleQuotes& other) {
    if (this != &other) {
      this->~StyleQuotes();
      new (this) StyleQuotes(other);
    }
    return *this;
  }
};

/// A generic value for the `drop-shadow()` filter and the `text-shadow` property.
///
/// Contrary to the canonical order from the spec, the color is serialised
/// first, like in Gecko and Webkit.
template<typename Color, typename SizeLength, typename ShapeLength>
struct StyleGenericSimpleShadow {
  /// Color.
  Color color;
  /// Horizontal radius.
  SizeLength horizontal;
  /// Vertical radius.
  SizeLength vertical;
  /// Blur radius.
  ShapeLength blur;

  bool operator==(const StyleGenericSimpleShadow& other) const {
    return color == other.color &&
           horizontal == other.horizontal &&
           vertical == other.vertical &&
           blur == other.blur;
  }
  bool operator!=(const StyleGenericSimpleShadow& other) const {
    return color != other.color ||
           horizontal != other.horizontal ||
           vertical != other.vertical ||
           blur != other.blur;
  }
};

/// A generic value for a single `box-shadow`.
template<typename Color, typename SizeLength, typename BlurShapeLength, typename ShapeLength>
struct StyleGenericBoxShadow {
  /// The base shadow.
  StyleGenericSimpleShadow<Color, SizeLength, BlurShapeLength> base;
  /// The spread radius.
  ShapeLength spread;
  /// Whether this is an inset box shadow.
  bool inset;

  bool operator==(const StyleGenericBoxShadow& other) const {
    return base == other.base &&
           spread == other.spread &&
           inset == other.inset;
  }
  bool operator!=(const StyleGenericBoxShadow& other) const {
    return base != other.base ||
           spread != other.spread ||
           inset != other.inset;
  }
};

/// A computed value for a single shadow of the `box-shadow` property.
using StyleBoxShadow = StyleGenericBoxShadow<StyleColor, StyleLength, StyleNonNegativeLength, StyleLength>;

/// A computed value for the `drop-shadow()` filter.
using StyleSimpleShadow = StyleGenericSimpleShadow<StyleColor, StyleLength, StyleNonNegativeLength>;

/// A generic value for a single side of a `border-image-width` property.
template<typename LP, typename N>
struct StyleGenericBorderImageSideWidth {
  enum class Tag : uint8_t {
    /// `<number>`
    ///
    /// NOTE: Numbers need to be before length-percentagess, in order to parse
    /// them first, since `0` should be a number, not the `0px` length.
    Number,
    /// `<length-or-percentage>`
    LengthPercentage,
    /// `auto`
    Auto,
  };

  struct StyleNumber_Body {
    N _0;

    bool operator==(const StyleNumber_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleNumber_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleLengthPercentage_Body {
    LP _0;

    bool operator==(const StyleLengthPercentage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLengthPercentage_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleNumber_Body number;
    StyleLengthPercentage_Body length_percentage;
  };

  static StyleGenericBorderImageSideWidth Number(const N &_0) {
    StyleGenericBorderImageSideWidth result;
    ::new (&result.number._0) (N)(_0);
    result.tag = Tag::Number;
    return result;
  }

  bool IsNumber() const {
    return tag == Tag::Number;
  }

  const N& AsNumber() const {
    MOZ_DIAGNOSTIC_ASSERT(IsNumber());
    return number._0;
  }

  static StyleGenericBorderImageSideWidth LengthPercentage(const LP &_0) {
    StyleGenericBorderImageSideWidth result;
    ::new (&result.length_percentage._0) (LP)(_0);
    result.tag = Tag::LengthPercentage;
    return result;
  }

  bool IsLengthPercentage() const {
    return tag == Tag::LengthPercentage;
  }

  const LP& AsLengthPercentage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLengthPercentage());
    return length_percentage._0;
  }

  static StyleGenericBorderImageSideWidth Auto() {
    StyleGenericBorderImageSideWidth result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  bool operator==(const StyleGenericBorderImageSideWidth& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Number: return number == other.number;
      case Tag::LengthPercentage: return length_percentage == other.length_percentage;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericBorderImageSideWidth& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericBorderImageSideWidth() {

  }
  public:


  ~StyleGenericBorderImageSideWidth() {
    switch (tag) {
      case Tag::Number: number.~StyleNumber_Body(); break;
      case Tag::LengthPercentage: length_percentage.~StyleLengthPercentage_Body(); break;
      default: break;
    }
  }

  StyleGenericBorderImageSideWidth(const StyleGenericBorderImageSideWidth& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Number: ::new (&number) (StyleNumber_Body)(other.number); break;
      case Tag::LengthPercentage: ::new (&length_percentage) (StyleLengthPercentage_Body)(other.length_percentage); break;
      default: break;
    }
  }
  StyleGenericBorderImageSideWidth& operator=(const StyleGenericBorderImageSideWidth& other) {
    if (this != &other) {
      this->~StyleGenericBorderImageSideWidth();
      new (this) StyleGenericBorderImageSideWidth(other);
    }
    return *this;
  }
};

/// A computed value for a single side of a `border-image-width` property.
using StyleBorderImageSideWidth = StyleGenericBorderImageSideWidth<StyleNonNegativeLengthPercentage, StyleNonNegativeNumber>;

/// A computed value for the `border-image-width` property.
using StyleBorderImageWidth = StyleRect<StyleBorderImageSideWidth>;

#if defined(CBINDGEN_IS_GECKO)
/// A specified non-image `url()` value.
using StyleSpecifiedUrl = StyleCssUrl;
#endif

#if defined(CBINDGEN_IS_SERVO)
/// A specified url() value for servo.
using StyleSpecifiedUrl = StyleCssUrl;
#endif

#if defined(CBINDGEN_IS_GECKO)
/// The computed value of a CSS non-image `url()`.
///
/// The only difference between specified and computed URLs is the
/// serialization.
struct StyleComputedUrl {
  StyleSpecifiedUrl _0;

  bool operator==(const StyleComputedUrl& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleComputedUrl& other) const {
    return _0 != other._0;
  }
  // Forwarded from CssUrl.
  inline nsDependentCSubstring SpecifiedSerialization() const;
  inline const URLExtraData& ExtraData() const;
  inline nsIURI* GetURI() const;
  inline StyleLoadData& LoadData() const;

  inline bool IsLocalRef() const;
  inline bool HasRef() const;
  inline StyleCorsMode CorsMode() const;
  already_AddRefed<nsIURI> ResolveLocalRef(nsIURI* aBase) const;
  already_AddRefed<nsIURI> ResolveLocalRef(const nsIContent* aContent) const;

  // Only relevant for images.
  inline bool IsImageResolved() const;
  inline imgRequestProxy* GetImage() const;
  void ResolveImage(dom::Document&, const StyleComputedUrl* aOldImage);
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// The computed value of a CSS image `url()`.
using StyleComputedImageUrl = StyleComputedUrl;
#endif

/// An image url or none, used for example in list-style-image
template<typename U>
struct StyleGenericUrlOrNone {
  enum class Tag : uint8_t {
    /// `none`
    None,
    /// A URL.
    Url,
  };

  struct StyleUrl_Body {
    U _0;

    bool operator==(const StyleUrl_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleUrl_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleUrl_Body url;
  };

  static StyleGenericUrlOrNone None() {
    StyleGenericUrlOrNone result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericUrlOrNone Url(const U &_0) {
    StyleGenericUrlOrNone result;
    ::new (&result.url._0) (U)(_0);
    result.tag = Tag::Url;
    return result;
  }

  bool IsUrl() const {
    return tag == Tag::Url;
  }

  const U& AsUrl() const {
    MOZ_DIAGNOSTIC_ASSERT(IsUrl());
    return url._0;
  }

  bool operator==(const StyleGenericUrlOrNone& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Url: return url == other.url;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericUrlOrNone& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericUrlOrNone() {

  }
  public:


  ~StyleGenericUrlOrNone() {
    switch (tag) {
      case Tag::Url: url.~StyleUrl_Body(); break;
      default: break;
    }
  }

  StyleGenericUrlOrNone(const StyleGenericUrlOrNone& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Url: ::new (&url) (StyleUrl_Body)(other.url); break;
      default: break;
    }
  }
  StyleGenericUrlOrNone& operator=(const StyleGenericUrlOrNone& other) {
    if (this != &other) {
      this->~StyleGenericUrlOrNone();
      new (this) StyleGenericUrlOrNone(other);
    }
    return *this;
  }
};

/// Computed <url> | <none>
using StyleUrlOrNone = StyleGenericUrlOrNone<StyleComputedUrl>;

/// A wrapper of values between zero and one.
template<typename T>
using StyleZeroToOne = T;

/// A wrapper of Number, but the value between 0 and 1
using StyleZeroToOneNumber = StyleZeroToOne<StyleCSSFloat>;

/// A generic value for a single `filter`.
template<typename Angle, typename NonNegativeFactor, typename ZeroToOneFactor, typename Length, typename Shadow, typename U>
struct StyleGenericFilter {
  enum class Tag : uint8_t {
    /// `blur(<length>)`
    Blur,
    /// `brightness(<factor>)`
    Brightness,
    /// `contrast(<factor>)`
    Contrast,
    /// `grayscale(<factor>)`
    Grayscale,
    /// `hue-rotate(<angle>)`
    HueRotate,
    /// `invert(<factor>)`
    Invert,
    /// `opacity(<factor>)`
    Opacity,
    /// `saturate(<factor>)`
    Saturate,
    /// `sepia(<factor>)`
    Sepia,
    /// `drop-shadow(...)`
    DropShadow,
    /// `<url>`
    Url,
  };

  struct StyleBlur_Body {
    Length _0;

    bool operator==(const StyleBlur_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleBlur_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleBrightness_Body {
    NonNegativeFactor _0;

    bool operator==(const StyleBrightness_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleBrightness_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleContrast_Body {
    NonNegativeFactor _0;

    bool operator==(const StyleContrast_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleContrast_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleGrayscale_Body {
    ZeroToOneFactor _0;

    bool operator==(const StyleGrayscale_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleGrayscale_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleHueRotate_Body {
    Angle _0;

    bool operator==(const StyleHueRotate_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleHueRotate_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleInvert_Body {
    ZeroToOneFactor _0;

    bool operator==(const StyleInvert_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleInvert_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleOpacity_Body {
    ZeroToOneFactor _0;

    bool operator==(const StyleOpacity_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleOpacity_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleSaturate_Body {
    NonNegativeFactor _0;

    bool operator==(const StyleSaturate_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleSaturate_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleSepia_Body {
    ZeroToOneFactor _0;

    bool operator==(const StyleSepia_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleSepia_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleDropShadow_Body {
    Shadow _0;

    bool operator==(const StyleDropShadow_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleDropShadow_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleUrl_Body {
    U _0;

    bool operator==(const StyleUrl_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleUrl_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleBlur_Body blur;
    StyleBrightness_Body brightness;
    StyleContrast_Body contrast;
    StyleGrayscale_Body grayscale;
    StyleHueRotate_Body hue_rotate;
    StyleInvert_Body invert;
    StyleOpacity_Body opacity;
    StyleSaturate_Body saturate;
    StyleSepia_Body sepia;
    StyleDropShadow_Body drop_shadow;
    StyleUrl_Body url;
  };

  static StyleGenericFilter Blur(const Length &_0) {
    StyleGenericFilter result;
    ::new (&result.blur._0) (Length)(_0);
    result.tag = Tag::Blur;
    return result;
  }

  bool IsBlur() const {
    return tag == Tag::Blur;
  }

  const Length& AsBlur() const {
    MOZ_DIAGNOSTIC_ASSERT(IsBlur());
    return blur._0;
  }

  static StyleGenericFilter Brightness(const NonNegativeFactor &_0) {
    StyleGenericFilter result;
    ::new (&result.brightness._0) (NonNegativeFactor)(_0);
    result.tag = Tag::Brightness;
    return result;
  }

  bool IsBrightness() const {
    return tag == Tag::Brightness;
  }

  const NonNegativeFactor& AsBrightness() const {
    MOZ_DIAGNOSTIC_ASSERT(IsBrightness());
    return brightness._0;
  }

  static StyleGenericFilter Contrast(const NonNegativeFactor &_0) {
    StyleGenericFilter result;
    ::new (&result.contrast._0) (NonNegativeFactor)(_0);
    result.tag = Tag::Contrast;
    return result;
  }

  bool IsContrast() const {
    return tag == Tag::Contrast;
  }

  const NonNegativeFactor& AsContrast() const {
    MOZ_DIAGNOSTIC_ASSERT(IsContrast());
    return contrast._0;
  }

  static StyleGenericFilter Grayscale(const ZeroToOneFactor &_0) {
    StyleGenericFilter result;
    ::new (&result.grayscale._0) (ZeroToOneFactor)(_0);
    result.tag = Tag::Grayscale;
    return result;
  }

  bool IsGrayscale() const {
    return tag == Tag::Grayscale;
  }

  const ZeroToOneFactor& AsGrayscale() const {
    MOZ_DIAGNOSTIC_ASSERT(IsGrayscale());
    return grayscale._0;
  }

  static StyleGenericFilter HueRotate(const Angle &_0) {
    StyleGenericFilter result;
    ::new (&result.hue_rotate._0) (Angle)(_0);
    result.tag = Tag::HueRotate;
    return result;
  }

  bool IsHueRotate() const {
    return tag == Tag::HueRotate;
  }

  const Angle& AsHueRotate() const {
    MOZ_DIAGNOSTIC_ASSERT(IsHueRotate());
    return hue_rotate._0;
  }

  static StyleGenericFilter Invert(const ZeroToOneFactor &_0) {
    StyleGenericFilter result;
    ::new (&result.invert._0) (ZeroToOneFactor)(_0);
    result.tag = Tag::Invert;
    return result;
  }

  bool IsInvert() const {
    return tag == Tag::Invert;
  }

  const ZeroToOneFactor& AsInvert() const {
    MOZ_DIAGNOSTIC_ASSERT(IsInvert());
    return invert._0;
  }

  static StyleGenericFilter Opacity(const ZeroToOneFactor &_0) {
    StyleGenericFilter result;
    ::new (&result.opacity._0) (ZeroToOneFactor)(_0);
    result.tag = Tag::Opacity;
    return result;
  }

  bool IsOpacity() const {
    return tag == Tag::Opacity;
  }

  const ZeroToOneFactor& AsOpacity() const {
    MOZ_DIAGNOSTIC_ASSERT(IsOpacity());
    return opacity._0;
  }

  static StyleGenericFilter Saturate(const NonNegativeFactor &_0) {
    StyleGenericFilter result;
    ::new (&result.saturate._0) (NonNegativeFactor)(_0);
    result.tag = Tag::Saturate;
    return result;
  }

  bool IsSaturate() const {
    return tag == Tag::Saturate;
  }

  const NonNegativeFactor& AsSaturate() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSaturate());
    return saturate._0;
  }

  static StyleGenericFilter Sepia(const ZeroToOneFactor &_0) {
    StyleGenericFilter result;
    ::new (&result.sepia._0) (ZeroToOneFactor)(_0);
    result.tag = Tag::Sepia;
    return result;
  }

  bool IsSepia() const {
    return tag == Tag::Sepia;
  }

  const ZeroToOneFactor& AsSepia() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSepia());
    return sepia._0;
  }

  static StyleGenericFilter DropShadow(const Shadow &_0) {
    StyleGenericFilter result;
    ::new (&result.drop_shadow._0) (Shadow)(_0);
    result.tag = Tag::DropShadow;
    return result;
  }

  bool IsDropShadow() const {
    return tag == Tag::DropShadow;
  }

  const Shadow& AsDropShadow() const {
    MOZ_DIAGNOSTIC_ASSERT(IsDropShadow());
    return drop_shadow._0;
  }

  static StyleGenericFilter Url(const U &_0) {
    StyleGenericFilter result;
    ::new (&result.url._0) (U)(_0);
    result.tag = Tag::Url;
    return result;
  }

  bool IsUrl() const {
    return tag == Tag::Url;
  }

  const U& AsUrl() const {
    MOZ_DIAGNOSTIC_ASSERT(IsUrl());
    return url._0;
  }

  bool operator==(const StyleGenericFilter& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Blur: return blur == other.blur;
      case Tag::Brightness: return brightness == other.brightness;
      case Tag::Contrast: return contrast == other.contrast;
      case Tag::Grayscale: return grayscale == other.grayscale;
      case Tag::HueRotate: return hue_rotate == other.hue_rotate;
      case Tag::Invert: return invert == other.invert;
      case Tag::Opacity: return opacity == other.opacity;
      case Tag::Saturate: return saturate == other.saturate;
      case Tag::Sepia: return sepia == other.sepia;
      case Tag::DropShadow: return drop_shadow == other.drop_shadow;
      case Tag::Url: return url == other.url;

    }
    return true;
  }

  bool operator!=(const StyleGenericFilter& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericFilter() {

  }
  public:


  ~StyleGenericFilter() {
    switch (tag) {
      case Tag::Blur: blur.~StyleBlur_Body(); break;
      case Tag::Brightness: brightness.~StyleBrightness_Body(); break;
      case Tag::Contrast: contrast.~StyleContrast_Body(); break;
      case Tag::Grayscale: grayscale.~StyleGrayscale_Body(); break;
      case Tag::HueRotate: hue_rotate.~StyleHueRotate_Body(); break;
      case Tag::Invert: invert.~StyleInvert_Body(); break;
      case Tag::Opacity: opacity.~StyleOpacity_Body(); break;
      case Tag::Saturate: saturate.~StyleSaturate_Body(); break;
      case Tag::Sepia: sepia.~StyleSepia_Body(); break;
      case Tag::DropShadow: drop_shadow.~StyleDropShadow_Body(); break;
      case Tag::Url: url.~StyleUrl_Body(); break;

    }
  }

  StyleGenericFilter(const StyleGenericFilter& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Blur: ::new (&blur) (StyleBlur_Body)(other.blur); break;
      case Tag::Brightness: ::new (&brightness) (StyleBrightness_Body)(other.brightness); break;
      case Tag::Contrast: ::new (&contrast) (StyleContrast_Body)(other.contrast); break;
      case Tag::Grayscale: ::new (&grayscale) (StyleGrayscale_Body)(other.grayscale); break;
      case Tag::HueRotate: ::new (&hue_rotate) (StyleHueRotate_Body)(other.hue_rotate); break;
      case Tag::Invert: ::new (&invert) (StyleInvert_Body)(other.invert); break;
      case Tag::Opacity: ::new (&opacity) (StyleOpacity_Body)(other.opacity); break;
      case Tag::Saturate: ::new (&saturate) (StyleSaturate_Body)(other.saturate); break;
      case Tag::Sepia: ::new (&sepia) (StyleSepia_Body)(other.sepia); break;
      case Tag::DropShadow: ::new (&drop_shadow) (StyleDropShadow_Body)(other.drop_shadow); break;
      case Tag::Url: ::new (&url) (StyleUrl_Body)(other.url); break;

    }
  }
  StyleGenericFilter& operator=(const StyleGenericFilter& other) {
    if (this != &other) {
      this->~StyleGenericFilter();
      new (this) StyleGenericFilter(other);
    }
    return *this;
  }
};

#if defined(CBINDGEN_IS_GECKO)
/// A computed value for a single `filter`.
using StyleFilter = StyleGenericFilter<StyleAngle, StyleNonNegativeNumber, StyleZeroToOneNumber, StyleNonNegativeLength, StyleSimpleShadow, StyleComputedUrl>;
#endif

#if defined(CBINDGEN_IS_SERVO)
/// A computed value for a single `filter`.
using StyleFilter = StyleGenericFilter<StyleAngle, StyleNonNegativeNumber, StyleZeroToOneNumber, StyleNonNegativeLength, StyleImpossible, StyleImpossible>;
#endif

/// A computed gradient line direction.
struct StyleLineDirection {
  enum class Tag : uint8_t {
    /// An angle.
    Angle,
    /// A horizontal direction.
    Horizontal,
    /// A vertical direction.
    Vertical,
    /// A corner.
    Corner,
  };

  struct StyleAngle_Body {
    StyleAngle _0;

    bool operator==(const StyleAngle_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleAngle_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleHorizontal_Body {
    StyleHorizontalPositionKeyword _0;

    bool operator==(const StyleHorizontal_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleHorizontal_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleVertical_Body {
    StyleVerticalPositionKeyword _0;

    bool operator==(const StyleVertical_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleVertical_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleCorner_Body {
    StyleHorizontalPositionKeyword _0;
    StyleVerticalPositionKeyword _1;

    bool operator==(const StyleCorner_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const StyleCorner_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  Tag tag;
  union {
    StyleAngle_Body angle;
    StyleHorizontal_Body horizontal;
    StyleVertical_Body vertical;
    StyleCorner_Body corner;
  };

  static StyleLineDirection Angle(const StyleAngle &_0) {
    StyleLineDirection result;
    ::new (&result.angle._0) (StyleAngle)(_0);
    result.tag = Tag::Angle;
    return result;
  }

  bool IsAngle() const {
    return tag == Tag::Angle;
  }

  const StyleAngle& AsAngle() const {
    MOZ_DIAGNOSTIC_ASSERT(IsAngle());
    return angle._0;
  }

  static StyleLineDirection Horizontal(const StyleHorizontalPositionKeyword &_0) {
    StyleLineDirection result;
    ::new (&result.horizontal._0) (StyleHorizontalPositionKeyword)(_0);
    result.tag = Tag::Horizontal;
    return result;
  }

  bool IsHorizontal() const {
    return tag == Tag::Horizontal;
  }

  const StyleHorizontalPositionKeyword& AsHorizontal() const {
    MOZ_DIAGNOSTIC_ASSERT(IsHorizontal());
    return horizontal._0;
  }

  static StyleLineDirection Vertical(const StyleVerticalPositionKeyword &_0) {
    StyleLineDirection result;
    ::new (&result.vertical._0) (StyleVerticalPositionKeyword)(_0);
    result.tag = Tag::Vertical;
    return result;
  }

  bool IsVertical() const {
    return tag == Tag::Vertical;
  }

  const StyleVerticalPositionKeyword& AsVertical() const {
    MOZ_DIAGNOSTIC_ASSERT(IsVertical());
    return vertical._0;
  }

  static StyleLineDirection Corner(const StyleHorizontalPositionKeyword &_0,
                                   const StyleVerticalPositionKeyword &_1) {
    StyleLineDirection result;
    ::new (&result.corner._0) (StyleHorizontalPositionKeyword)(_0);
    ::new (&result.corner._1) (StyleVerticalPositionKeyword)(_1);
    result.tag = Tag::Corner;
    return result;
  }

  bool IsCorner() const {
    return tag == Tag::Corner;
  }

  const StyleCorner_Body& AsCorner() const {
    MOZ_DIAGNOSTIC_ASSERT(IsCorner());
    return corner;
  }

  bool operator==(const StyleLineDirection& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Angle: return angle == other.angle;
      case Tag::Horizontal: return horizontal == other.horizontal;
      case Tag::Vertical: return vertical == other.vertical;
      case Tag::Corner: return corner == other.corner;

    }
    return true;
  }

  bool operator!=(const StyleLineDirection& other) const {
    return !(*this == other);
  }

  private:
  StyleLineDirection() {

  }
  public:


  ~StyleLineDirection() {
    switch (tag) {
      case Tag::Angle: angle.~StyleAngle_Body(); break;
      case Tag::Horizontal: horizontal.~StyleHorizontal_Body(); break;
      case Tag::Vertical: vertical.~StyleVertical_Body(); break;
      case Tag::Corner: corner.~StyleCorner_Body(); break;

    }
  }

  StyleLineDirection(const StyleLineDirection& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Angle: ::new (&angle) (StyleAngle_Body)(other.angle); break;
      case Tag::Horizontal: ::new (&horizontal) (StyleHorizontal_Body)(other.horizontal); break;
      case Tag::Vertical: ::new (&vertical) (StyleVertical_Body)(other.vertical); break;
      case Tag::Corner: ::new (&corner) (StyleCorner_Body)(other.corner); break;

    }
  }
  StyleLineDirection& operator=(const StyleLineDirection& other) {
    if (this != &other) {
      this->~StyleLineDirection();
      new (this) StyleLineDirection(other);
    }
    return *this;
  }
};

struct StyleAngleOrPercentage {
  enum class Tag : uint8_t {
    Percentage,
    Angle,
  };

  struct StylePercentage_Body {
    StylePercentage _0;

    bool operator==(const StylePercentage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StylePercentage_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleAngle_Body {
    StyleAngle _0;

    bool operator==(const StyleAngle_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleAngle_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StylePercentage_Body percentage;
    StyleAngle_Body angle;
  };

  static StyleAngleOrPercentage Percentage(const StylePercentage &_0) {
    StyleAngleOrPercentage result;
    ::new (&result.percentage._0) (StylePercentage)(_0);
    result.tag = Tag::Percentage;
    return result;
  }

  bool IsPercentage() const {
    return tag == Tag::Percentage;
  }

  const StylePercentage& AsPercentage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPercentage());
    return percentage._0;
  }

  static StyleAngleOrPercentage Angle(const StyleAngle &_0) {
    StyleAngleOrPercentage result;
    ::new (&result.angle._0) (StyleAngle)(_0);
    result.tag = Tag::Angle;
    return result;
  }

  bool IsAngle() const {
    return tag == Tag::Angle;
  }

  const StyleAngle& AsAngle() const {
    MOZ_DIAGNOSTIC_ASSERT(IsAngle());
    return angle._0;
  }

  bool operator==(const StyleAngleOrPercentage& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Percentage: return percentage == other.percentage;
      case Tag::Angle: return angle == other.angle;

    }
    return true;
  }

  bool operator!=(const StyleAngleOrPercentage& other) const {
    return !(*this == other);
  }

  private:
  StyleAngleOrPercentage() {

  }
  public:


  ~StyleAngleOrPercentage() {
    switch (tag) {
      case Tag::Percentage: percentage.~StylePercentage_Body(); break;
      case Tag::Angle: angle.~StyleAngle_Body(); break;

    }
  }

  StyleAngleOrPercentage(const StyleAngleOrPercentage& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Percentage: ::new (&percentage) (StylePercentage_Body)(other.percentage); break;
      case Tag::Angle: ::new (&angle) (StyleAngle_Body)(other.angle); break;

    }
  }
  StyleAngleOrPercentage& operator=(const StyleAngleOrPercentage& other) {
    if (this != &other) {
      this->~StyleAngleOrPercentage();
      new (this) StyleAngleOrPercentage(other);
    }
    return *this;
  }
};

/// A gradient item.
/// <https://drafts.csswg.org/css-images-4/#color-stop-syntax>
template<typename Color, typename T>
struct StyleGenericGradientItem {
  enum class Tag : uint8_t {
    /// A simple color stop, without position.
    SimpleColorStop,
    /// A complex color stop, with a position.
    ComplexColorStop,
    /// An interpolation hint.
    InterpolationHint,
  };

  struct StyleSimpleColorStop_Body {
    Color _0;

    bool operator==(const StyleSimpleColorStop_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleSimpleColorStop_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleComplexColorStop_Body {
    /// The color for the stop.
    Color color;
    /// The position for the stop.
    T position;

    bool operator==(const StyleComplexColorStop_Body& other) const {
      return color == other.color &&
             position == other.position;
    }
    bool operator!=(const StyleComplexColorStop_Body& other) const {
      return color != other.color ||
             position != other.position;
    }
  };

  struct StyleInterpolationHint_Body {
    T _0;

    bool operator==(const StyleInterpolationHint_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleInterpolationHint_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleSimpleColorStop_Body simple_color_stop;
    StyleComplexColorStop_Body complex_color_stop;
    StyleInterpolationHint_Body interpolation_hint;
  };

  static StyleGenericGradientItem SimpleColorStop(const Color &_0) {
    StyleGenericGradientItem result;
    ::new (&result.simple_color_stop._0) (Color)(_0);
    result.tag = Tag::SimpleColorStop;
    return result;
  }

  bool IsSimpleColorStop() const {
    return tag == Tag::SimpleColorStop;
  }

  const Color& AsSimpleColorStop() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSimpleColorStop());
    return simple_color_stop._0;
  }

  static StyleGenericGradientItem ComplexColorStop(const Color &color,
                                                   const T &position) {
    StyleGenericGradientItem result;
    ::new (&result.complex_color_stop.color) (Color)(color);
    ::new (&result.complex_color_stop.position) (T)(position);
    result.tag = Tag::ComplexColorStop;
    return result;
  }

  bool IsComplexColorStop() const {
    return tag == Tag::ComplexColorStop;
  }

  const StyleComplexColorStop_Body& AsComplexColorStop() const {
    MOZ_DIAGNOSTIC_ASSERT(IsComplexColorStop());
    return complex_color_stop;
  }

  static StyleGenericGradientItem InterpolationHint(const T &_0) {
    StyleGenericGradientItem result;
    ::new (&result.interpolation_hint._0) (T)(_0);
    result.tag = Tag::InterpolationHint;
    return result;
  }

  bool IsInterpolationHint() const {
    return tag == Tag::InterpolationHint;
  }

  const T& AsInterpolationHint() const {
    MOZ_DIAGNOSTIC_ASSERT(IsInterpolationHint());
    return interpolation_hint._0;
  }

  bool operator==(const StyleGenericGradientItem& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::SimpleColorStop: return simple_color_stop == other.simple_color_stop;
      case Tag::ComplexColorStop: return complex_color_stop == other.complex_color_stop;
      case Tag::InterpolationHint: return interpolation_hint == other.interpolation_hint;

    }
    return true;
  }

  bool operator!=(const StyleGenericGradientItem& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericGradientItem() {

  }
  public:


  ~StyleGenericGradientItem() {
    switch (tag) {
      case Tag::SimpleColorStop: simple_color_stop.~StyleSimpleColorStop_Body(); break;
      case Tag::ComplexColorStop: complex_color_stop.~StyleComplexColorStop_Body(); break;
      case Tag::InterpolationHint: interpolation_hint.~StyleInterpolationHint_Body(); break;

    }
  }

  StyleGenericGradientItem(const StyleGenericGradientItem& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::SimpleColorStop: ::new (&simple_color_stop) (StyleSimpleColorStop_Body)(other.simple_color_stop); break;
      case Tag::ComplexColorStop: ::new (&complex_color_stop) (StyleComplexColorStop_Body)(other.complex_color_stop); break;
      case Tag::InterpolationHint: ::new (&interpolation_hint) (StyleInterpolationHint_Body)(other.interpolation_hint); break;

    }
  }
  StyleGenericGradientItem& operator=(const StyleGenericGradientItem& other) {
    if (this != &other) {
      this->~StyleGenericGradientItem();
      new (this) StyleGenericGradientItem(other);
    }
    return *this;
  }
};

/// A circle shape.
template<typename NonNegativeLength>
struct StyleGenericCircle {
  enum class Tag : uint8_t {
    /// A circle radius.
    Radius,
    /// A circle extent.
    Extent,
  };

  struct StyleRadius_Body {
    NonNegativeLength _0;

    bool operator==(const StyleRadius_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleRadius_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleExtent_Body {
    StyleShapeExtent _0;

    bool operator==(const StyleExtent_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleExtent_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleRadius_Body radius;
    StyleExtent_Body extent;
  };

  static StyleGenericCircle Radius(const NonNegativeLength &_0) {
    StyleGenericCircle result;
    ::new (&result.radius._0) (NonNegativeLength)(_0);
    result.tag = Tag::Radius;
    return result;
  }

  bool IsRadius() const {
    return tag == Tag::Radius;
  }

  const NonNegativeLength& AsRadius() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRadius());
    return radius._0;
  }

  static StyleGenericCircle Extent(const StyleShapeExtent &_0) {
    StyleGenericCircle result;
    ::new (&result.extent._0) (StyleShapeExtent)(_0);
    result.tag = Tag::Extent;
    return result;
  }

  bool IsExtent() const {
    return tag == Tag::Extent;
  }

  const StyleShapeExtent& AsExtent() const {
    MOZ_DIAGNOSTIC_ASSERT(IsExtent());
    return extent._0;
  }

  bool operator==(const StyleGenericCircle& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Radius: return radius == other.radius;
      case Tag::Extent: return extent == other.extent;

    }
    return true;
  }

  bool operator!=(const StyleGenericCircle& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericCircle() {

  }
  public:


  ~StyleGenericCircle() {
    switch (tag) {
      case Tag::Radius: radius.~StyleRadius_Body(); break;
      case Tag::Extent: extent.~StyleExtent_Body(); break;

    }
  }

  StyleGenericCircle(const StyleGenericCircle& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Radius: ::new (&radius) (StyleRadius_Body)(other.radius); break;
      case Tag::Extent: ::new (&extent) (StyleExtent_Body)(other.extent); break;

    }
  }
  StyleGenericCircle& operator=(const StyleGenericCircle& other) {
    if (this != &other) {
      this->~StyleGenericCircle();
      new (this) StyleGenericCircle(other);
    }
    return *this;
  }
};

/// An ellipse shape.
template<typename NonNegativeLengthPercentage>
struct StyleGenericEllipse {
  enum class Tag : uint8_t {
    /// An ellipse pair of radii.
    Radii,
    /// An ellipse extent.
    Extent,
  };

  struct StyleRadii_Body {
    NonNegativeLengthPercentage _0;
    NonNegativeLengthPercentage _1;

    bool operator==(const StyleRadii_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const StyleRadii_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct StyleExtent_Body {
    StyleShapeExtent _0;

    bool operator==(const StyleExtent_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleExtent_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleRadii_Body radii;
    StyleExtent_Body extent;
  };

  static StyleGenericEllipse Radii(const NonNegativeLengthPercentage &_0,
                                   const NonNegativeLengthPercentage &_1) {
    StyleGenericEllipse result;
    ::new (&result.radii._0) (NonNegativeLengthPercentage)(_0);
    ::new (&result.radii._1) (NonNegativeLengthPercentage)(_1);
    result.tag = Tag::Radii;
    return result;
  }

  bool IsRadii() const {
    return tag == Tag::Radii;
  }

  const StyleRadii_Body& AsRadii() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRadii());
    return radii;
  }

  static StyleGenericEllipse Extent(const StyleShapeExtent &_0) {
    StyleGenericEllipse result;
    ::new (&result.extent._0) (StyleShapeExtent)(_0);
    result.tag = Tag::Extent;
    return result;
  }

  bool IsExtent() const {
    return tag == Tag::Extent;
  }

  const StyleShapeExtent& AsExtent() const {
    MOZ_DIAGNOSTIC_ASSERT(IsExtent());
    return extent._0;
  }

  bool operator==(const StyleGenericEllipse& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Radii: return radii == other.radii;
      case Tag::Extent: return extent == other.extent;

    }
    return true;
  }

  bool operator!=(const StyleGenericEllipse& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericEllipse() {

  }
  public:


  ~StyleGenericEllipse() {
    switch (tag) {
      case Tag::Radii: radii.~StyleRadii_Body(); break;
      case Tag::Extent: extent.~StyleExtent_Body(); break;

    }
  }

  StyleGenericEllipse(const StyleGenericEllipse& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Radii: ::new (&radii) (StyleRadii_Body)(other.radii); break;
      case Tag::Extent: ::new (&extent) (StyleExtent_Body)(other.extent); break;

    }
  }
  StyleGenericEllipse& operator=(const StyleGenericEllipse& other) {
    if (this != &other) {
      this->~StyleGenericEllipse();
      new (this) StyleGenericEllipse(other);
    }
    return *this;
  }
};

/// A radial gradient's ending shape.
template<typename NonNegativeLength, typename NonNegativeLengthPercentage>
struct StyleGenericEndingShape {
  enum class Tag : uint8_t {
    /// A circular gradient.
    Circle,
    /// An elliptic gradient.
    Ellipse,
  };

  struct StyleCircle_Body {
    StyleGenericCircle<NonNegativeLength> _0;

    bool operator==(const StyleCircle_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleCircle_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleEllipse_Body {
    StyleGenericEllipse<NonNegativeLengthPercentage> _0;

    bool operator==(const StyleEllipse_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleEllipse_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleCircle_Body circle;
    StyleEllipse_Body ellipse;
  };

  static StyleGenericEndingShape Circle(const StyleGenericCircle<NonNegativeLength> &_0) {
    StyleGenericEndingShape result;
    ::new (&result.circle._0) (StyleGenericCircle<NonNegativeLength>)(_0);
    result.tag = Tag::Circle;
    return result;
  }

  bool IsCircle() const {
    return tag == Tag::Circle;
  }

  const StyleGenericCircle<NonNegativeLength>& AsCircle() const {
    MOZ_DIAGNOSTIC_ASSERT(IsCircle());
    return circle._0;
  }

  static StyleGenericEndingShape Ellipse(const StyleGenericEllipse<NonNegativeLengthPercentage> &_0) {
    StyleGenericEndingShape result;
    ::new (&result.ellipse._0) (StyleGenericEllipse<NonNegativeLengthPercentage>)(_0);
    result.tag = Tag::Ellipse;
    return result;
  }

  bool IsEllipse() const {
    return tag == Tag::Ellipse;
  }

  const StyleGenericEllipse<NonNegativeLengthPercentage>& AsEllipse() const {
    MOZ_DIAGNOSTIC_ASSERT(IsEllipse());
    return ellipse._0;
  }

  bool operator==(const StyleGenericEndingShape& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Circle: return circle == other.circle;
      case Tag::Ellipse: return ellipse == other.ellipse;

    }
    return true;
  }

  bool operator!=(const StyleGenericEndingShape& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericEndingShape() {

  }
  public:


  ~StyleGenericEndingShape() {
    switch (tag) {
      case Tag::Circle: circle.~StyleCircle_Body(); break;
      case Tag::Ellipse: ellipse.~StyleEllipse_Body(); break;

    }
  }

  StyleGenericEndingShape(const StyleGenericEndingShape& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Circle: ::new (&circle) (StyleCircle_Body)(other.circle); break;
      case Tag::Ellipse: ::new (&ellipse) (StyleEllipse_Body)(other.ellipse); break;

    }
  }
  StyleGenericEndingShape& operator=(const StyleGenericEndingShape& other) {
    if (this != &other) {
      this->~StyleGenericEndingShape();
      new (this) StyleGenericEndingShape(other);
    }
    return *this;
  }
};

/// A CSS gradient.
/// <https://drafts.csswg.org/css-images/#gradients>
template<typename LineDirection, typename LengthPercentage, typename NonNegativeLength, typename NonNegativeLengthPercentage, typename Position, typename Angle, typename AngleOrPercentage, typename Color>
struct StyleGenericGradient {
  enum class Tag {
    /// A linear gradient.
    Linear,
    /// A radial gradient.
    Radial,
    /// A conic gradient.
    Conic,
  };

  struct StyleLinear_Body {
    /// Line direction
    LineDirection direction;
    /// The color stops and interpolation hints.
    StyleOwnedSlice<StyleGenericGradientItem<Color, LengthPercentage>> items;
    /// True if this is a repeating gradient.
    bool repeating;
    /// Compatibility mode.
    StyleGradientCompatMode compat_mode;

    bool operator==(const StyleLinear_Body& other) const {
      return direction == other.direction &&
             items == other.items &&
             repeating == other.repeating &&
             compat_mode == other.compat_mode;
    }
    bool operator!=(const StyleLinear_Body& other) const {
      return direction != other.direction ||
             items != other.items ||
             repeating != other.repeating ||
             compat_mode != other.compat_mode;
    }
  };

  struct StyleRadial_Body {
    /// Shape of gradient
    StyleGenericEndingShape<NonNegativeLength, NonNegativeLengthPercentage> shape;
    /// Center of gradient
    Position position;
    /// The color stops and interpolation hints.
    StyleOwnedSlice<StyleGenericGradientItem<Color, LengthPercentage>> items;
    /// True if this is a repeating gradient.
    bool repeating;
    /// Compatibility mode.
    StyleGradientCompatMode compat_mode;

    bool operator==(const StyleRadial_Body& other) const {
      return shape == other.shape &&
             position == other.position &&
             items == other.items &&
             repeating == other.repeating &&
             compat_mode == other.compat_mode;
    }
    bool operator!=(const StyleRadial_Body& other) const {
      return shape != other.shape ||
             position != other.position ||
             items != other.items ||
             repeating != other.repeating ||
             compat_mode != other.compat_mode;
    }
  };

  struct StyleConic_Body {
    /// Start angle of gradient
    Angle angle;
    /// Center of gradient
    Position position;
    /// The color stops and interpolation hints.
    StyleOwnedSlice<StyleGenericGradientItem<Color, AngleOrPercentage>> items;
    /// True if this is a repeating gradient.
    bool repeating;

    bool operator==(const StyleConic_Body& other) const {
      return angle == other.angle &&
             position == other.position &&
             items == other.items &&
             repeating == other.repeating;
    }
    bool operator!=(const StyleConic_Body& other) const {
      return angle != other.angle ||
             position != other.position ||
             items != other.items ||
             repeating != other.repeating;
    }
  };

  Tag tag;
  union {
    StyleLinear_Body linear;
    StyleRadial_Body radial;
    StyleConic_Body conic;
  };

  static StyleGenericGradient Linear(const LineDirection &direction,
                                     const StyleOwnedSlice<StyleGenericGradientItem<Color, LengthPercentage>> &items,
                                     const bool &repeating,
                                     const StyleGradientCompatMode &compat_mode) {
    StyleGenericGradient result;
    ::new (&result.linear.direction) (LineDirection)(direction);
    ::new (&result.linear.items) (StyleOwnedSlice<StyleGenericGradientItem<Color, LengthPercentage>>)(items);
    ::new (&result.linear.repeating) (bool)(repeating);
    ::new (&result.linear.compat_mode) (StyleGradientCompatMode)(compat_mode);
    result.tag = Tag::Linear;
    return result;
  }

  bool IsLinear() const {
    return tag == Tag::Linear;
  }

  const StyleLinear_Body& AsLinear() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLinear());
    return linear;
  }

  static StyleGenericGradient Radial(const StyleGenericEndingShape<NonNegativeLength, NonNegativeLengthPercentage> &shape,
                                     const Position &position,
                                     const StyleOwnedSlice<StyleGenericGradientItem<Color, LengthPercentage>> &items,
                                     const bool &repeating,
                                     const StyleGradientCompatMode &compat_mode) {
    StyleGenericGradient result;
    ::new (&result.radial.shape) (StyleGenericEndingShape<NonNegativeLength, NonNegativeLengthPercentage>)(shape);
    ::new (&result.radial.position) (Position)(position);
    ::new (&result.radial.items) (StyleOwnedSlice<StyleGenericGradientItem<Color, LengthPercentage>>)(items);
    ::new (&result.radial.repeating) (bool)(repeating);
    ::new (&result.radial.compat_mode) (StyleGradientCompatMode)(compat_mode);
    result.tag = Tag::Radial;
    return result;
  }

  bool IsRadial() const {
    return tag == Tag::Radial;
  }

  const StyleRadial_Body& AsRadial() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRadial());
    return radial;
  }

  static StyleGenericGradient Conic(const Angle &angle,
                                    const Position &position,
                                    const StyleOwnedSlice<StyleGenericGradientItem<Color, AngleOrPercentage>> &items,
                                    const bool &repeating) {
    StyleGenericGradient result;
    ::new (&result.conic.angle) (Angle)(angle);
    ::new (&result.conic.position) (Position)(position);
    ::new (&result.conic.items) (StyleOwnedSlice<StyleGenericGradientItem<Color, AngleOrPercentage>>)(items);
    ::new (&result.conic.repeating) (bool)(repeating);
    result.tag = Tag::Conic;
    return result;
  }

  bool IsConic() const {
    return tag == Tag::Conic;
  }

  const StyleConic_Body& AsConic() const {
    MOZ_DIAGNOSTIC_ASSERT(IsConic());
    return conic;
  }

  bool operator==(const StyleGenericGradient& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Linear: return linear == other.linear;
      case Tag::Radial: return radial == other.radial;
      case Tag::Conic: return conic == other.conic;

    }
    return true;
  }

  bool operator!=(const StyleGenericGradient& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericGradient() {

  }
  public:


  ~StyleGenericGradient() {
    switch (tag) {
      case Tag::Linear: linear.~StyleLinear_Body(); break;
      case Tag::Radial: radial.~StyleRadial_Body(); break;
      case Tag::Conic: conic.~StyleConic_Body(); break;

    }
  }

  StyleGenericGradient(const StyleGenericGradient& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Linear: ::new (&linear) (StyleLinear_Body)(other.linear); break;
      case Tag::Radial: ::new (&radial) (StyleRadial_Body)(other.radial); break;
      case Tag::Conic: ::new (&conic) (StyleConic_Body)(other.conic); break;

    }
  }
  StyleGenericGradient& operator=(const StyleGenericGradient& other) {
    if (this != &other) {
      this->~StyleGenericGradient();
      new (this) StyleGenericGradient(other);
    }
    return *this;
  }
  inline bool Repeating() const;
  bool IsOpaque() const;
};

/// Computed values for a CSS gradient.
/// <https://drafts.csswg.org/css-images/#gradients>
using StyleGradient = StyleGenericGradient<StyleLineDirection, StyleLengthPercentage, StyleNonNegativeLength, StyleNonNegativeLengthPercentage, StylePosition, StyleAngle, StyleAngleOrPercentage, StyleColor>;

/// A range of rows or columns. Using this instead of std::ops::Range for FFI
/// purposes.
struct StyleUnsignedRange {
  /// The start of the range.
  uint32_t start;
  /// The end of the range.
  uint32_t end;

  bool operator==(const StyleUnsignedRange& other) const {
    return start == other.start &&
           end == other.end;
  }
  bool operator!=(const StyleUnsignedRange& other) const {
    return start != other.start ||
           end != other.end;
  }
};

/// Not associated with any particular grid item, but can be referenced from the
/// grid-placement properties.
struct StyleNamedArea {
  /// Name of the `named area`
  StyleAtom name;
  /// Rows of the `named area`
  StyleUnsignedRange rows;
  /// Columns of the `named area`
  StyleUnsignedRange columns;

  bool operator==(const StyleNamedArea& other) const {
    return name == other.name &&
           rows == other.rows &&
           columns == other.columns;
  }
  bool operator!=(const StyleNamedArea& other) const {
    return name != other.name ||
           rows != other.rows ||
           columns != other.columns;
  }
};

/// https://drafts.csswg.org/css-grid/#named-grid-area
struct StyleTemplateAreas {
  /// `named area` containing for each template area
  StyleOwnedSlice<StyleNamedArea> areas;
  /// The original CSS string value of each template area
  StyleOwnedSlice<StyleOwnedStr> strings;
  /// The number of columns of the grid.
  uint32_t width;

  bool operator==(const StyleTemplateAreas& other) const {
    return areas == other.areas &&
           strings == other.strings &&
           width == other.width;
  }
  bool operator!=(const StyleTemplateAreas& other) const {
    return areas != other.areas ||
           strings != other.strings ||
           width != other.width;
  }
};

/// Arc type for `Arc<TemplateAreas>`
using StyleTemplateAreasArc = StyleArc<StyleTemplateAreas>;

/// This property specifies named grid areas.
///
/// The syntax of this property also provides a visualization of the structure
/// of the grid, making the overall layout of the grid container easier to
/// understand.
struct StyleGridTemplateAreas {
  enum class Tag : uint8_t {
    /// The `none` value.
    None,
    /// The actual value.
    Areas,
  };

  struct StyleAreas_Body {
    StyleTemplateAreasArc _0;

    bool operator==(const StyleAreas_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleAreas_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleAreas_Body areas;
  };

  static StyleGridTemplateAreas None() {
    StyleGridTemplateAreas result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGridTemplateAreas Areas(const StyleTemplateAreasArc &_0) {
    StyleGridTemplateAreas result;
    ::new (&result.areas._0) (StyleTemplateAreasArc)(_0);
    result.tag = Tag::Areas;
    return result;
  }

  bool IsAreas() const {
    return tag == Tag::Areas;
  }

  const StyleTemplateAreasArc& AsAreas() const {
    MOZ_DIAGNOSTIC_ASSERT(IsAreas());
    return areas._0;
  }

  bool operator==(const StyleGridTemplateAreas& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Areas: return areas == other.areas;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGridTemplateAreas& other) const {
    return !(*this == other);
  }

  private:
  StyleGridTemplateAreas() {

  }
  public:


  ~StyleGridTemplateAreas() {
    switch (tag) {
      case Tag::Areas: areas.~StyleAreas_Body(); break;
      default: break;
    }
  }

  StyleGridTemplateAreas(const StyleGridTemplateAreas& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Areas: ::new (&areas) (StyleAreas_Body)(other.areas); break;
      default: break;
    }
  }
  StyleGridTemplateAreas& operator=(const StyleGridTemplateAreas& other) {
    if (this != &other) {
      this->~StyleGridTemplateAreas();
      new (this) StyleGridTemplateAreas(other);
    }
    return *this;
  }
};

/// A `<grid-line>` type.
///
/// <https://drafts.csswg.org/css-grid/#typedef-grid-row-start-grid-line>
template<typename Integer>
struct StyleGenericGridLine {
  /// A custom identifier for named lines, or the empty atom otherwise.
  ///
  /// <https://drafts.csswg.org/css-grid/#grid-placement-slot>
  StyleAtom ident;
  /// Denotes the nth grid line from grid item's placement.
  ///
  /// This is clamped by MIN_GRID_LINE and MAX_GRID_LINE.
  ///
  /// NOTE(emilio): If we ever allow animating these we need to either do
  /// something more complicated for the clamping, or do this clamping at
  /// used-value time.
  Integer line_num;
  /// Flag to check whether it's a `span` keyword.
  bool is_span;

  bool operator==(const StyleGenericGridLine& other) const {
    return ident == other.ident &&
           line_num == other.line_num &&
           is_span == other.is_span;
  }
  bool operator!=(const StyleGenericGridLine& other) const {
    return ident != other.ident ||
           line_num != other.line_num ||
           is_span != other.is_span;
  }
  // Returns the `auto` value.
  inline StyleGenericGridLine();
  inline bool IsAuto() const;
  // The line name, or nsGkAtoms::_empty if not present.
  inline nsAtom* LineName() const;
};

/// The computed value of a `<grid-line>`.
using StyleGridLine = StyleGenericGridLine<StyleInteger>;

/// A track breadth for explicit grid track sizing. It's generic solely to
/// avoid re-implementing it for the computed type.
///
/// <https://drafts.csswg.org/css-grid/#typedef-track-breadth>
template<typename L>
struct StyleGenericTrackBreadth {
  enum class Tag : uint8_t {
    /// The generic type is almost always a non-negative `<length-percentage>`
    Breadth,
    /// A flex fraction specified in `fr` units.
    Fr,
    /// `auto`
    Auto,
    /// `min-content`
    MinContent,
    /// `max-content`
    MaxContent,
  };

  struct StyleBreadth_Body {
    L _0;

    bool operator==(const StyleBreadth_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleBreadth_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleFr_Body {
    StyleCSSFloat _0;

    bool operator==(const StyleFr_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleFr_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleBreadth_Body breadth;
    StyleFr_Body fr;
  };

  static StyleGenericTrackBreadth Breadth(const L &_0) {
    StyleGenericTrackBreadth result;
    ::new (&result.breadth._0) (L)(_0);
    result.tag = Tag::Breadth;
    return result;
  }

  bool IsBreadth() const {
    return tag == Tag::Breadth;
  }

  const L& AsBreadth() const {
    MOZ_DIAGNOSTIC_ASSERT(IsBreadth());
    return breadth._0;
  }

  static StyleGenericTrackBreadth Fr(const StyleCSSFloat &_0) {
    StyleGenericTrackBreadth result;
    ::new (&result.fr._0) (StyleCSSFloat)(_0);
    result.tag = Tag::Fr;
    return result;
  }

  bool IsFr() const {
    return tag == Tag::Fr;
  }

  const StyleCSSFloat& AsFr() const {
    MOZ_DIAGNOSTIC_ASSERT(IsFr());
    return fr._0;
  }

  static StyleGenericTrackBreadth Auto() {
    StyleGenericTrackBreadth result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  static StyleGenericTrackBreadth MinContent() {
    StyleGenericTrackBreadth result;
    result.tag = Tag::MinContent;
    return result;
  }

  bool IsMinContent() const {
    return tag == Tag::MinContent;
  }

  static StyleGenericTrackBreadth MaxContent() {
    StyleGenericTrackBreadth result;
    result.tag = Tag::MaxContent;
    return result;
  }

  bool IsMaxContent() const {
    return tag == Tag::MaxContent;
  }

  bool operator==(const StyleGenericTrackBreadth& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Breadth: return breadth == other.breadth;
      case Tag::Fr: return fr == other.fr;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericTrackBreadth& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericTrackBreadth() {

  }
  public:


  ~StyleGenericTrackBreadth() {
    switch (tag) {
      case Tag::Breadth: breadth.~StyleBreadth_Body(); break;
      case Tag::Fr: fr.~StyleFr_Body(); break;
      default: break;
    }
  }

  StyleGenericTrackBreadth(const StyleGenericTrackBreadth& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Breadth: ::new (&breadth) (StyleBreadth_Body)(other.breadth); break;
      case Tag::Fr: ::new (&fr) (StyleFr_Body)(other.fr); break;
      default: break;
    }
  }
  StyleGenericTrackBreadth& operator=(const StyleGenericTrackBreadth& other) {
    if (this != &other) {
      this->~StyleGenericTrackBreadth();
      new (this) StyleGenericTrackBreadth(other);
    }
    return *this;
  }
  inline bool HasPercent() const;
};

/// A `<track-size>` type for explicit grid track sizing. Like `<track-breadth>`, this is
/// generic only to avoid code bloat. It only takes `<length-percentage>`
///
/// <https://drafts.csswg.org/css-grid/#typedef-track-size>
template<typename L>
struct StyleGenericTrackSize {
  enum class Tag : uint8_t {
    /// A flexible `<track-breadth>`
    Breadth,
    /// A `minmax` function for a range over an inflexible `<track-breadth>`
    /// and a flexible `<track-breadth>`
    ///
    /// <https://drafts.csswg.org/css-grid/#valdef-grid-template-columns-minmax>
    Minmax,
    /// A `fit-content` function.
    ///
    /// This stores a TrackBreadth<L> for convenience, but it can only be a
    /// LengthPercentage.
    ///
    /// <https://drafts.csswg.org/css-grid/#valdef-grid-template-columns-fit-content>
    FitContent,
  };

  struct StyleBreadth_Body {
    StyleGenericTrackBreadth<L> _0;

    bool operator==(const StyleBreadth_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleBreadth_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleMinmax_Body {
    StyleGenericTrackBreadth<L> _0;
    StyleGenericTrackBreadth<L> _1;

    bool operator==(const StyleMinmax_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const StyleMinmax_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct StyleFitContent_Body {
    StyleGenericTrackBreadth<L> _0;

    bool operator==(const StyleFitContent_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleFitContent_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleBreadth_Body breadth;
    StyleMinmax_Body minmax;
    StyleFitContent_Body fit_content;
  };

  static StyleGenericTrackSize Breadth(const StyleGenericTrackBreadth<L> &_0) {
    StyleGenericTrackSize result;
    ::new (&result.breadth._0) (StyleGenericTrackBreadth<L>)(_0);
    result.tag = Tag::Breadth;
    return result;
  }

  bool IsBreadth() const {
    return tag == Tag::Breadth;
  }

  const StyleGenericTrackBreadth<L>& AsBreadth() const {
    MOZ_DIAGNOSTIC_ASSERT(IsBreadth());
    return breadth._0;
  }

  static StyleGenericTrackSize Minmax(const StyleGenericTrackBreadth<L> &_0,
                                      const StyleGenericTrackBreadth<L> &_1) {
    StyleGenericTrackSize result;
    ::new (&result.minmax._0) (StyleGenericTrackBreadth<L>)(_0);
    ::new (&result.minmax._1) (StyleGenericTrackBreadth<L>)(_1);
    result.tag = Tag::Minmax;
    return result;
  }

  bool IsMinmax() const {
    return tag == Tag::Minmax;
  }

  const StyleMinmax_Body& AsMinmax() const {
    MOZ_DIAGNOSTIC_ASSERT(IsMinmax());
    return minmax;
  }

  static StyleGenericTrackSize FitContent(const StyleGenericTrackBreadth<L> &_0) {
    StyleGenericTrackSize result;
    ::new (&result.fit_content._0) (StyleGenericTrackBreadth<L>)(_0);
    result.tag = Tag::FitContent;
    return result;
  }

  bool IsFitContent() const {
    return tag == Tag::FitContent;
  }

  const StyleGenericTrackBreadth<L>& AsFitContent() const {
    MOZ_DIAGNOSTIC_ASSERT(IsFitContent());
    return fit_content._0;
  }

  bool operator==(const StyleGenericTrackSize& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Breadth: return breadth == other.breadth;
      case Tag::Minmax: return minmax == other.minmax;
      case Tag::FitContent: return fit_content == other.fit_content;

    }
    return true;
  }

  bool operator!=(const StyleGenericTrackSize& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericTrackSize() {

  }
  public:


  ~StyleGenericTrackSize() {
    switch (tag) {
      case Tag::Breadth: breadth.~StyleBreadth_Body(); break;
      case Tag::Minmax: minmax.~StyleMinmax_Body(); break;
      case Tag::FitContent: fit_content.~StyleFitContent_Body(); break;

    }
  }

  StyleGenericTrackSize(const StyleGenericTrackSize& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Breadth: ::new (&breadth) (StyleBreadth_Body)(other.breadth); break;
      case Tag::Minmax: ::new (&minmax) (StyleMinmax_Body)(other.minmax); break;
      case Tag::FitContent: ::new (&fit_content) (StyleFitContent_Body)(other.fit_content); break;

    }
  }
  StyleGenericTrackSize& operator=(const StyleGenericTrackSize& other) {
    if (this != &other) {
      this->~StyleGenericTrackSize();
      new (this) StyleGenericTrackSize(other);
    }
    return *this;
  }
  // Implemented in nsGridContainerFrame.cpp
  inline const StyleGenericTrackBreadth<L>& GetMin() const;
  inline const StyleGenericTrackBreadth<L>& GetMax() const;
};

/// The computed value of a grid `<track-size>`
using StyleTrackSize = StyleGenericTrackSize<StyleLengthPercentage>;

/// The computed value of a grid `<track-breadth>`
using StyleTrackBreadth = StyleGenericTrackBreadth<StyleLengthPercentage>;

/// A `<track-size>+`.
/// We use the empty slice as `auto`, and always parse `auto` as an empty slice.
/// This means it's impossible to have a slice containing only one auto item.
template<typename T>
using StyleGenericImplicitGridTracks = StyleOwnedSlice<T>;

/// The computed value of a grid `<track-size>+`
using StyleImplicitGridTracks = StyleGenericImplicitGridTracks<StyleTrackSize>;

/// An SVG paint value without the fallback.
///
/// Whereas the spec only allows PaintServer to have a fallback, Gecko lets the
/// context properties have a fallback as well.
template<typename C, typename U>
struct StyleGenericSVGPaintKind {
  enum class Tag : uint8_t {
    /// `none`
    None,
    /// `<color>`
    Color,
    /// `url(...)`
    PaintServer,
    /// `context-fill`
    ContextFill,
    /// `context-stroke`
    ContextStroke,
  };

  struct StyleColor_Body {
    C _0;

    bool operator==(const StyleColor_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleColor_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StylePaintServer_Body {
    U _0;

    bool operator==(const StylePaintServer_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StylePaintServer_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleColor_Body color;
    StylePaintServer_Body paint_server;
  };

  static StyleGenericSVGPaintKind None() {
    StyleGenericSVGPaintKind result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericSVGPaintKind Color(const C &_0) {
    StyleGenericSVGPaintKind result;
    ::new (&result.color._0) (C)(_0);
    result.tag = Tag::Color;
    return result;
  }

  bool IsColor() const {
    return tag == Tag::Color;
  }

  const C& AsColor() const {
    MOZ_DIAGNOSTIC_ASSERT(IsColor());
    return color._0;
  }

  static StyleGenericSVGPaintKind PaintServer(const U &_0) {
    StyleGenericSVGPaintKind result;
    ::new (&result.paint_server._0) (U)(_0);
    result.tag = Tag::PaintServer;
    return result;
  }

  bool IsPaintServer() const {
    return tag == Tag::PaintServer;
  }

  const U& AsPaintServer() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPaintServer());
    return paint_server._0;
  }

  static StyleGenericSVGPaintKind ContextFill() {
    StyleGenericSVGPaintKind result;
    result.tag = Tag::ContextFill;
    return result;
  }

  bool IsContextFill() const {
    return tag == Tag::ContextFill;
  }

  static StyleGenericSVGPaintKind ContextStroke() {
    StyleGenericSVGPaintKind result;
    result.tag = Tag::ContextStroke;
    return result;
  }

  bool IsContextStroke() const {
    return tag == Tag::ContextStroke;
  }

  bool operator==(const StyleGenericSVGPaintKind& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Color: return color == other.color;
      case Tag::PaintServer: return paint_server == other.paint_server;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericSVGPaintKind& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericSVGPaintKind() {

  }
  public:


  ~StyleGenericSVGPaintKind() {
    switch (tag) {
      case Tag::Color: color.~StyleColor_Body(); break;
      case Tag::PaintServer: paint_server.~StylePaintServer_Body(); break;
      default: break;
    }
  }

  StyleGenericSVGPaintKind(const StyleGenericSVGPaintKind& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Color: ::new (&color) (StyleColor_Body)(other.color); break;
      case Tag::PaintServer: ::new (&paint_server) (StylePaintServer_Body)(other.paint_server); break;
      default: break;
    }
  }
  StyleGenericSVGPaintKind& operator=(const StyleGenericSVGPaintKind& other) {
    if (this != &other) {
      this->~StyleGenericSVGPaintKind();
      new (this) StyleGenericSVGPaintKind(other);
    }
    return *this;
  }
};

/// The fallback of an SVG paint server value.
template<typename C>
struct StyleGenericSVGPaintFallback {
  enum class Tag : uint8_t {
    /// The `none` keyword.
    None,
    /// A magic value that represents no fallback specified and serializes to
    /// the empty string.
    Unset,
    /// A color.
    Color,
  };

  struct StyleColor_Body {
    C _0;

    bool operator==(const StyleColor_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleColor_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleColor_Body color;
  };

  static StyleGenericSVGPaintFallback None() {
    StyleGenericSVGPaintFallback result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericSVGPaintFallback Unset() {
    StyleGenericSVGPaintFallback result;
    result.tag = Tag::Unset;
    return result;
  }

  bool IsUnset() const {
    return tag == Tag::Unset;
  }

  static StyleGenericSVGPaintFallback Color(const C &_0) {
    StyleGenericSVGPaintFallback result;
    ::new (&result.color._0) (C)(_0);
    result.tag = Tag::Color;
    return result;
  }

  bool IsColor() const {
    return tag == Tag::Color;
  }

  const C& AsColor() const {
    MOZ_DIAGNOSTIC_ASSERT(IsColor());
    return color._0;
  }

  bool operator==(const StyleGenericSVGPaintFallback& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Color: return color == other.color;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericSVGPaintFallback& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericSVGPaintFallback() {

  }
  public:


  ~StyleGenericSVGPaintFallback() {
    switch (tag) {
      case Tag::Color: color.~StyleColor_Body(); break;
      default: break;
    }
  }

  StyleGenericSVGPaintFallback(const StyleGenericSVGPaintFallback& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Color: ::new (&color) (StyleColor_Body)(other.color); break;
      default: break;
    }
  }
  StyleGenericSVGPaintFallback& operator=(const StyleGenericSVGPaintFallback& other) {
    if (this != &other) {
      this->~StyleGenericSVGPaintFallback();
      new (this) StyleGenericSVGPaintFallback(other);
    }
    return *this;
  }
};

/// An SVG paint value
///
/// <https://www.w3.org/TR/SVG2/painting.html#SpecifyingPaint>
template<typename Color, typename Url>
struct StyleGenericSVGPaint {
  /// The paint source.
  StyleGenericSVGPaintKind<Color, Url> kind;
  /// The fallback color.
  StyleGenericSVGPaintFallback<Color> fallback;

  bool operator==(const StyleGenericSVGPaint& other) const {
    return kind == other.kind &&
           fallback == other.fallback;
  }
  bool operator!=(const StyleGenericSVGPaint& other) const {
    return kind != other.kind ||
           fallback != other.fallback;
  }
};

/// Computed SVG Paint value
using StyleSVGPaint = StyleGenericSVGPaint<StyleColor, StyleComputedUrl>;

/// Computed SVG Paint Kind value
using StyleSVGPaintKind = StyleGenericSVGPaintKind<StyleColor, StyleComputedUrl>;

/// The initial argument of the `repeat` function.
///
/// <https://drafts.csswg.org/css-grid/#typedef-track-repeat>
template<typename Integer>
struct StyleRepeatCount {
  enum class Tag : uint8_t {
    /// A positive integer. This is allowed only for `<track-repeat>` and `<fixed-repeat>`
    Number,
    /// An `<auto-fill>` keyword allowed only for `<auto-repeat>`
    AutoFill,
    /// An `<auto-fit>` keyword allowed only for `<auto-repeat>`
    AutoFit,
  };

  struct StyleNumber_Body {
    Integer _0;

    bool operator==(const StyleNumber_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleNumber_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleNumber_Body number;
  };

  static StyleRepeatCount Number(const Integer &_0) {
    StyleRepeatCount result;
    ::new (&result.number._0) (Integer)(_0);
    result.tag = Tag::Number;
    return result;
  }

  bool IsNumber() const {
    return tag == Tag::Number;
  }

  const Integer& AsNumber() const {
    MOZ_DIAGNOSTIC_ASSERT(IsNumber());
    return number._0;
  }

  static StyleRepeatCount AutoFill() {
    StyleRepeatCount result;
    result.tag = Tag::AutoFill;
    return result;
  }

  bool IsAutoFill() const {
    return tag == Tag::AutoFill;
  }

  static StyleRepeatCount AutoFit() {
    StyleRepeatCount result;
    result.tag = Tag::AutoFit;
    return result;
  }

  bool IsAutoFit() const {
    return tag == Tag::AutoFit;
  }

  bool operator==(const StyleRepeatCount& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Number: return number == other.number;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleRepeatCount& other) const {
    return !(*this == other);
  }

  private:
  StyleRepeatCount() {

  }
  public:


  ~StyleRepeatCount() {
    switch (tag) {
      case Tag::Number: number.~StyleNumber_Body(); break;
      default: break;
    }
  }

  StyleRepeatCount(const StyleRepeatCount& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Number: ::new (&number) (StyleNumber_Body)(other.number); break;
      default: break;
    }
  }
  StyleRepeatCount& operator=(const StyleRepeatCount& other) {
    if (this != &other) {
      this->~StyleRepeatCount();
      new (this) StyleRepeatCount(other);
    }
    return *this;
  }
};

/// The structure containing `<line-names>` and `<track-size>` values.
///
/// It can also hold `repeat()` function parameters, which expands into the respective
/// values in its computed form.
template<typename L, typename I>
struct StyleGenericTrackRepeat {
  /// The number of times for the value to be repeated (could also be `auto-fit` or `auto-fill`)
  StyleRepeatCount<I> count;
  /// `<line-names>` accompanying `<track_size>` values.
  ///
  /// If there's no `<line-names>`, then it's represented by an empty vector.
  /// For N `<track-size>` values, there will be N+1 `<line-names>`, and so this vector's
  /// length is always one value more than that of the `<track-size>`.
  StyleOwnedSlice<StyleOwnedSlice<StyleCustomIdent>> line_names;
  /// `<track-size>` values.
  StyleOwnedSlice<StyleGenericTrackSize<L>> track_sizes;

  bool operator==(const StyleGenericTrackRepeat& other) const {
    return count == other.count &&
           line_names == other.line_names &&
           track_sizes == other.track_sizes;
  }
  bool operator!=(const StyleGenericTrackRepeat& other) const {
    return count != other.count ||
           line_names != other.line_names ||
           track_sizes != other.track_sizes;
  }
};

/// Track list values. Can be <track-size> or <track-repeat>
template<typename LengthPercentage, typename Integer>
struct StyleGenericTrackListValue {
  enum class Tag : uint8_t {
    /// A <track-size> value.
    TrackSize,
    /// A <track-repeat> value.
    TrackRepeat,
  };

  struct StyleTrackSize_Body {
    StyleGenericTrackSize<LengthPercentage> _0;

    bool operator==(const StyleTrackSize_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleTrackSize_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleTrackRepeat_Body {
    StyleGenericTrackRepeat<LengthPercentage, Integer> _0;

    bool operator==(const StyleTrackRepeat_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleTrackRepeat_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleTrackSize_Body track_size;
    StyleTrackRepeat_Body track_repeat;
  };

  static StyleGenericTrackListValue TrackSize(const StyleGenericTrackSize<LengthPercentage> &_0) {
    StyleGenericTrackListValue result;
    ::new (&result.track_size._0) (StyleGenericTrackSize<LengthPercentage>)(_0);
    result.tag = Tag::TrackSize;
    return result;
  }

  bool IsTrackSize() const {
    return tag == Tag::TrackSize;
  }

  const StyleGenericTrackSize<LengthPercentage>& AsTrackSize() const {
    MOZ_DIAGNOSTIC_ASSERT(IsTrackSize());
    return track_size._0;
  }

  static StyleGenericTrackListValue TrackRepeat(const StyleGenericTrackRepeat<LengthPercentage, Integer> &_0) {
    StyleGenericTrackListValue result;
    ::new (&result.track_repeat._0) (StyleGenericTrackRepeat<LengthPercentage, Integer>)(_0);
    result.tag = Tag::TrackRepeat;
    return result;
  }

  bool IsTrackRepeat() const {
    return tag == Tag::TrackRepeat;
  }

  const StyleGenericTrackRepeat<LengthPercentage, Integer>& AsTrackRepeat() const {
    MOZ_DIAGNOSTIC_ASSERT(IsTrackRepeat());
    return track_repeat._0;
  }

  bool operator==(const StyleGenericTrackListValue& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::TrackSize: return track_size == other.track_size;
      case Tag::TrackRepeat: return track_repeat == other.track_repeat;

    }
    return true;
  }

  bool operator!=(const StyleGenericTrackListValue& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericTrackListValue() {

  }
  public:


  ~StyleGenericTrackListValue() {
    switch (tag) {
      case Tag::TrackSize: track_size.~StyleTrackSize_Body(); break;
      case Tag::TrackRepeat: track_repeat.~StyleTrackRepeat_Body(); break;

    }
  }

  StyleGenericTrackListValue(const StyleGenericTrackListValue& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::TrackSize: ::new (&track_size) (StyleTrackSize_Body)(other.track_size); break;
      case Tag::TrackRepeat: ::new (&track_repeat) (StyleTrackRepeat_Body)(other.track_repeat); break;

    }
  }
  StyleGenericTrackListValue& operator=(const StyleGenericTrackListValue& other) {
    if (this != &other) {
      this->~StyleGenericTrackListValue();
      new (this) StyleGenericTrackListValue(other);
    }
    return *this;
  }
};

/// A grid `<track-list>` type.
///
/// <https://drafts.csswg.org/css-grid/#typedef-track-list>
template<typename LengthPercentage, typename Integer>
struct StyleGenericTrackList {
  /// The index in `values` where our `<auto-repeat>` value is, if in bounds.
  uintptr_t auto_repeat_index;
  /// A vector of `<track-size> | <track-repeat>` values.
  StyleOwnedSlice<StyleGenericTrackListValue<LengthPercentage, Integer>> values;
  /// `<line-names>` accompanying `<track-size> | <track-repeat>` values.
  ///
  /// If there's no `<line-names>`, then it's represented by an empty vector.
  /// For N values, there will be N+1 `<line-names>`, and so this vector's
  /// length is always one value more than that of the `<track-size>`.
  StyleOwnedSlice<StyleOwnedSlice<StyleCustomIdent>> line_names;

  bool operator==(const StyleGenericTrackList& other) const {
    return auto_repeat_index == other.auto_repeat_index &&
           values == other.values &&
           line_names == other.line_names;
  }
  bool operator!=(const StyleGenericTrackList& other) const {
    return auto_repeat_index != other.auto_repeat_index ||
           values != other.values ||
           line_names != other.line_names;
  }
};

/// The `<line-name-list>` for subgrids.
///
/// `subgrid [ <line-names> | repeat(<positive-integer> | auto-fill, <line-names>+) ]+`
///
/// https://drafts.csswg.org/css-grid-2/#typedef-line-name-list
struct StyleLineNameList {
  /// The optional `<line-name-list>`
  StyleOwnedSlice<StyleOwnedSlice<StyleCustomIdent>> names;
  /// Indicates the starting line names that requires `auto-fill`, if in bounds.
  uintptr_t fill_start;
  /// Indicates the number of line names in the auto-fill
  uintptr_t fill_len;

  bool operator==(const StyleLineNameList& other) const {
    return names == other.names &&
           fill_start == other.fill_start &&
           fill_len == other.fill_len;
  }
  bool operator!=(const StyleLineNameList& other) const {
    return names != other.names ||
           fill_start != other.fill_start ||
           fill_len != other.fill_len;
  }
};

/// Variants for `<grid-template-rows> | <grid-template-columns>`
template<typename L, typename I>
struct StyleGenericGridTemplateComponent {
  enum class Tag : uint8_t {
    /// `none` value.
    None,
    /// The grid `<track-list>`
    TrackList,
    /// A `subgrid <line-name-list>?`
    /// TODO: Support animations for this after subgrid is addressed in [grid-2] spec.
    Subgrid,
    /// `masonry` value.
    /// https://github.com/w3c/csswg-drafts/issues/4650
    Masonry,
  };

  struct StyleTrackList_Body {
    StyleBox<StyleGenericTrackList<L, I>> _0;

    bool operator==(const StyleTrackList_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleTrackList_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleSubgrid_Body {
    StyleBox<StyleLineNameList> _0;

    bool operator==(const StyleSubgrid_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleSubgrid_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleTrackList_Body track_list;
    StyleSubgrid_Body subgrid;
  };

  static StyleGenericGridTemplateComponent None() {
    StyleGenericGridTemplateComponent result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericGridTemplateComponent TrackList(const StyleBox<StyleGenericTrackList<L, I>> &_0) {
    StyleGenericGridTemplateComponent result;
    ::new (&result.track_list._0) (StyleBox<StyleGenericTrackList<L, I>>)(_0);
    result.tag = Tag::TrackList;
    return result;
  }

  bool IsTrackList() const {
    return tag == Tag::TrackList;
  }

  const StyleBox<StyleGenericTrackList<L, I>>& AsTrackList() const {
    MOZ_DIAGNOSTIC_ASSERT(IsTrackList());
    return track_list._0;
  }

  static StyleGenericGridTemplateComponent Subgrid(const StyleBox<StyleLineNameList> &_0) {
    StyleGenericGridTemplateComponent result;
    ::new (&result.subgrid._0) (StyleBox<StyleLineNameList>)(_0);
    result.tag = Tag::Subgrid;
    return result;
  }

  bool IsSubgrid() const {
    return tag == Tag::Subgrid;
  }

  const StyleBox<StyleLineNameList>& AsSubgrid() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSubgrid());
    return subgrid._0;
  }

  static StyleGenericGridTemplateComponent Masonry() {
    StyleGenericGridTemplateComponent result;
    result.tag = Tag::Masonry;
    return result;
  }

  bool IsMasonry() const {
    return tag == Tag::Masonry;
  }

  bool operator==(const StyleGenericGridTemplateComponent& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::TrackList: return track_list == other.track_list;
      case Tag::Subgrid: return subgrid == other.subgrid;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericGridTemplateComponent& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericGridTemplateComponent() {

  }
  public:


  ~StyleGenericGridTemplateComponent() {
    switch (tag) {
      case Tag::TrackList: track_list.~StyleTrackList_Body(); break;
      case Tag::Subgrid: subgrid.~StyleSubgrid_Body(); break;
      default: break;
    }
  }

  StyleGenericGridTemplateComponent(const StyleGenericGridTemplateComponent& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::TrackList: ::new (&track_list) (StyleTrackList_Body)(other.track_list); break;
      case Tag::Subgrid: ::new (&subgrid) (StyleSubgrid_Body)(other.subgrid); break;
      default: break;
    }
  }
  StyleGenericGridTemplateComponent& operator=(const StyleGenericGridTemplateComponent& other) {
    if (this != &other) {
      this->~StyleGenericGridTemplateComponent();
      new (this) StyleGenericGridTemplateComponent(other);
    }
    return *this;
  }
  inline Maybe<size_t> RepeatAutoIndex() const;
  inline const StyleGenericTrackRepeat<L, I>* GetRepeatAutoValue() const;
  inline bool HasRepeatAuto() const;
  inline Span<const StyleOwnedSlice<StyleCustomIdent>> LineNameLists(bool aIsSubgrid) const;
  inline Span<const StyleGenericTrackListValue<L, I>> TrackListValues() const;
};

/// `<grid-template-rows> | <grid-template-columns>`
using StyleGridTemplateComponent = StyleGenericGridTemplateComponent<StyleLengthPercentage, StyleInteger>;

/// Computed value for the text-emphasis-style property
struct StyleTextEmphasisStyle {
  enum class Tag : uint8_t {
    /// [ <fill> || <shape> ]
    Keyword,
    /// `none`
    None,
    /// `<string>` (of which only the first grapheme cluster will be used).
    String,
  };

  struct StyleKeyword_Body {
    StyleTextEmphasisFillMode fill;
    StyleTextEmphasisShapeKeyword shape;

    bool operator==(const StyleKeyword_Body& other) const {
      return fill == other.fill &&
             shape == other.shape;
    }
    bool operator!=(const StyleKeyword_Body& other) const {
      return fill != other.fill ||
             shape != other.shape;
    }
  };

  struct StyleString_Body {
    StyleOwnedStr _0;

    bool operator==(const StyleString_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleString_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleKeyword_Body keyword;
    StyleString_Body string;
  };

  static StyleTextEmphasisStyle Keyword(const StyleTextEmphasisFillMode &fill,
                                        const StyleTextEmphasisShapeKeyword &shape) {
    StyleTextEmphasisStyle result;
    ::new (&result.keyword.fill) (StyleTextEmphasisFillMode)(fill);
    ::new (&result.keyword.shape) (StyleTextEmphasisShapeKeyword)(shape);
    result.tag = Tag::Keyword;
    return result;
  }

  bool IsKeyword() const {
    return tag == Tag::Keyword;
  }

  const StyleKeyword_Body& AsKeyword() const {
    MOZ_DIAGNOSTIC_ASSERT(IsKeyword());
    return keyword;
  }

  static StyleTextEmphasisStyle None() {
    StyleTextEmphasisStyle result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleTextEmphasisStyle String(const StyleOwnedStr &_0) {
    StyleTextEmphasisStyle result;
    ::new (&result.string._0) (StyleOwnedStr)(_0);
    result.tag = Tag::String;
    return result;
  }

  bool IsString() const {
    return tag == Tag::String;
  }

  const StyleOwnedStr& AsString() const {
    MOZ_DIAGNOSTIC_ASSERT(IsString());
    return string._0;
  }

  bool operator==(const StyleTextEmphasisStyle& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Keyword: return keyword == other.keyword;
      case Tag::String: return string == other.string;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleTextEmphasisStyle& other) const {
    return !(*this == other);
  }

  private:
  StyleTextEmphasisStyle() {

  }
  public:


  ~StyleTextEmphasisStyle() {
    switch (tag) {
      case Tag::Keyword: keyword.~StyleKeyword_Body(); break;
      case Tag::String: string.~StyleString_Body(); break;
      default: break;
    }
  }

  StyleTextEmphasisStyle(const StyleTextEmphasisStyle& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Keyword: ::new (&keyword) (StyleKeyword_Body)(other.keyword); break;
      case Tag::String: ::new (&string) (StyleString_Body)(other.string); break;
      default: break;
    }
  }
  StyleTextEmphasisStyle& operator=(const StyleTextEmphasisStyle& other) {
    if (this != &other) {
      this->~StyleTextEmphasisStyle();
      new (this) StyleTextEmphasisStyle(other);
    }
    return *this;
  }
};

/// Set of variant alternates
struct StyleVariantAlternates {
  enum class Tag : uint8_t {
    /// Enables display of stylistic alternates
    Stylistic,
    /// Enables display with stylistic sets
    Styleset,
    /// Enables display of specific character variants
    CharacterVariant,
    /// Enables display of swash glyphs
    Swash,
    /// Enables replacement of default glyphs with ornaments
    Ornaments,
    /// Enables display of alternate annotation forms
    Annotation,
    /// Enables display of historical forms
    HistoricalForms,
  };

  struct StyleStylistic_Body {
    StyleCustomIdent _0;

    bool operator==(const StyleStylistic_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleStylistic_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleStyleset_Body {
    StyleOwnedSlice<StyleCustomIdent> _0;

    bool operator==(const StyleStyleset_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleStyleset_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleCharacterVariant_Body {
    StyleOwnedSlice<StyleCustomIdent> _0;

    bool operator==(const StyleCharacterVariant_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleCharacterVariant_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleSwash_Body {
    StyleCustomIdent _0;

    bool operator==(const StyleSwash_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleSwash_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleOrnaments_Body {
    StyleCustomIdent _0;

    bool operator==(const StyleOrnaments_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleOrnaments_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleAnnotation_Body {
    StyleCustomIdent _0;

    bool operator==(const StyleAnnotation_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleAnnotation_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleStylistic_Body stylistic;
    StyleStyleset_Body styleset;
    StyleCharacterVariant_Body character_variant;
    StyleSwash_Body swash;
    StyleOrnaments_Body ornaments;
    StyleAnnotation_Body annotation;
  };

  static StyleVariantAlternates Stylistic(const StyleCustomIdent &_0) {
    StyleVariantAlternates result;
    ::new (&result.stylistic._0) (StyleCustomIdent)(_0);
    result.tag = Tag::Stylistic;
    return result;
  }

  bool IsStylistic() const {
    return tag == Tag::Stylistic;
  }

  const StyleCustomIdent& AsStylistic() const {
    MOZ_DIAGNOSTIC_ASSERT(IsStylistic());
    return stylistic._0;
  }

  static StyleVariantAlternates Styleset(const StyleOwnedSlice<StyleCustomIdent> &_0) {
    StyleVariantAlternates result;
    ::new (&result.styleset._0) (StyleOwnedSlice<StyleCustomIdent>)(_0);
    result.tag = Tag::Styleset;
    return result;
  }

  bool IsStyleset() const {
    return tag == Tag::Styleset;
  }

  const StyleOwnedSlice<StyleCustomIdent>& AsStyleset() const {
    MOZ_DIAGNOSTIC_ASSERT(IsStyleset());
    return styleset._0;
  }

  static StyleVariantAlternates CharacterVariant(const StyleOwnedSlice<StyleCustomIdent> &_0) {
    StyleVariantAlternates result;
    ::new (&result.character_variant._0) (StyleOwnedSlice<StyleCustomIdent>)(_0);
    result.tag = Tag::CharacterVariant;
    return result;
  }

  bool IsCharacterVariant() const {
    return tag == Tag::CharacterVariant;
  }

  const StyleOwnedSlice<StyleCustomIdent>& AsCharacterVariant() const {
    MOZ_DIAGNOSTIC_ASSERT(IsCharacterVariant());
    return character_variant._0;
  }

  static StyleVariantAlternates Swash(const StyleCustomIdent &_0) {
    StyleVariantAlternates result;
    ::new (&result.swash._0) (StyleCustomIdent)(_0);
    result.tag = Tag::Swash;
    return result;
  }

  bool IsSwash() const {
    return tag == Tag::Swash;
  }

  const StyleCustomIdent& AsSwash() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSwash());
    return swash._0;
  }

  static StyleVariantAlternates Ornaments(const StyleCustomIdent &_0) {
    StyleVariantAlternates result;
    ::new (&result.ornaments._0) (StyleCustomIdent)(_0);
    result.tag = Tag::Ornaments;
    return result;
  }

  bool IsOrnaments() const {
    return tag == Tag::Ornaments;
  }

  const StyleCustomIdent& AsOrnaments() const {
    MOZ_DIAGNOSTIC_ASSERT(IsOrnaments());
    return ornaments._0;
  }

  static StyleVariantAlternates Annotation(const StyleCustomIdent &_0) {
    StyleVariantAlternates result;
    ::new (&result.annotation._0) (StyleCustomIdent)(_0);
    result.tag = Tag::Annotation;
    return result;
  }

  bool IsAnnotation() const {
    return tag == Tag::Annotation;
  }

  const StyleCustomIdent& AsAnnotation() const {
    MOZ_DIAGNOSTIC_ASSERT(IsAnnotation());
    return annotation._0;
  }

  static StyleVariantAlternates HistoricalForms() {
    StyleVariantAlternates result;
    result.tag = Tag::HistoricalForms;
    return result;
  }

  bool IsHistoricalForms() const {
    return tag == Tag::HistoricalForms;
  }

  bool operator==(const StyleVariantAlternates& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Stylistic: return stylistic == other.stylistic;
      case Tag::Styleset: return styleset == other.styleset;
      case Tag::CharacterVariant: return character_variant == other.character_variant;
      case Tag::Swash: return swash == other.swash;
      case Tag::Ornaments: return ornaments == other.ornaments;
      case Tag::Annotation: return annotation == other.annotation;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleVariantAlternates& other) const {
    return !(*this == other);
  }

  private:
  StyleVariantAlternates() {

  }
  public:


  ~StyleVariantAlternates() {
    switch (tag) {
      case Tag::Stylistic: stylistic.~StyleStylistic_Body(); break;
      case Tag::Styleset: styleset.~StyleStyleset_Body(); break;
      case Tag::CharacterVariant: character_variant.~StyleCharacterVariant_Body(); break;
      case Tag::Swash: swash.~StyleSwash_Body(); break;
      case Tag::Ornaments: ornaments.~StyleOrnaments_Body(); break;
      case Tag::Annotation: annotation.~StyleAnnotation_Body(); break;
      default: break;
    }
  }

  StyleVariantAlternates(const StyleVariantAlternates& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Stylistic: ::new (&stylistic) (StyleStylistic_Body)(other.stylistic); break;
      case Tag::Styleset: ::new (&styleset) (StyleStyleset_Body)(other.styleset); break;
      case Tag::CharacterVariant: ::new (&character_variant) (StyleCharacterVariant_Body)(other.character_variant); break;
      case Tag::Swash: ::new (&swash) (StyleSwash_Body)(other.swash); break;
      case Tag::Ornaments: ::new (&ornaments) (StyleOrnaments_Body)(other.ornaments); break;
      case Tag::Annotation: ::new (&annotation) (StyleAnnotation_Body)(other.annotation); break;
      default: break;
    }
  }
  StyleVariantAlternates& operator=(const StyleVariantAlternates& other) {
    if (this != &other) {
      this->~StyleVariantAlternates();
      new (this) StyleVariantAlternates(other);
    }
    return *this;
  }
};

/// List of Variant Alternates
using StyleVariantAlternatesList = StyleOwnedSlice<StyleVariantAlternates>;

/// The specified value is tree `PaintOrder` values packed into the
/// bitfields below, as a six-bit field, of 3 two-bit pairs
///
/// Each pair can be set to FILL, STROKE, or MARKERS
/// Lowest significant bit pairs are highest priority.
///  `normal` is the empty bitfield. The three pairs are
/// never zero in any case other than `normal`.
///
/// Higher priority values, i.e. the values specified first,
/// will be painted first (and may be covered by paintings of lower priority)
using StyleSVGPaintOrder = uint8_t;

/// A clip rect for clip and image-region
template<typename LengthOrAuto>
struct StyleGenericClipRect {
  LengthOrAuto top;
  LengthOrAuto right;
  LengthOrAuto bottom;
  LengthOrAuto left;

  bool operator==(const StyleGenericClipRect& other) const {
    return top == other.top &&
           right == other.right &&
           bottom == other.bottom &&
           left == other.left;
  }
  bool operator!=(const StyleGenericClipRect& other) const {
    return top != other.top ||
           right != other.right ||
           bottom != other.bottom ||
           left != other.left;
  }
  // Get the layout rect, replacing auto right / bottom values for aAutoSize.
  inline nsRect ToLayoutRect(nscoord aAutoSize = NS_MAXSIZE) const;
};

/// rect(...) | auto
using StyleClipRect = StyleGenericClipRect<StyleLengthOrAuto>;

/// Either a clip-rect or `auto`.
template<typename R>
struct StyleGenericClipRectOrAuto {
  enum class Tag : uint8_t {
    Auto,
    Rect,
  };

  struct StyleRect_Body {
    R _0;

    bool operator==(const StyleRect_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleRect_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleRect_Body rect;
  };

  static StyleGenericClipRectOrAuto Auto() {
    StyleGenericClipRectOrAuto result;
    result.tag = Tag::Auto;
    return result;
  }

  bool IsAuto() const {
    return tag == Tag::Auto;
  }

  static StyleGenericClipRectOrAuto Rect(const R &_0) {
    StyleGenericClipRectOrAuto result;
    ::new (&result.rect._0) (R)(_0);
    result.tag = Tag::Rect;
    return result;
  }

  bool IsRect() const {
    return tag == Tag::Rect;
  }

  const R& AsRect() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRect());
    return rect._0;
  }

  bool operator==(const StyleGenericClipRectOrAuto& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Rect: return rect == other.rect;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericClipRectOrAuto& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericClipRectOrAuto() {

  }
  public:


  ~StyleGenericClipRectOrAuto() {
    switch (tag) {
      case Tag::Rect: rect.~StyleRect_Body(); break;
      default: break;
    }
  }

  StyleGenericClipRectOrAuto(const StyleGenericClipRectOrAuto& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Rect: ::new (&rect) (StyleRect_Body)(other.rect); break;
      default: break;
    }
  }
  StyleGenericClipRectOrAuto& operator=(const StyleGenericClipRectOrAuto& other) {
    if (this != &other) {
      this->~StyleGenericClipRectOrAuto();
      new (this) StyleGenericClipRectOrAuto(other);
    }
    return *this;
  }
};

/// rect(...) | auto
using StyleClipRectOrAuto = StyleGenericClipRectOrAuto<StyleClipRect>;

/// A name / value pair for counters.
template<typename Integer>
struct StyleGenericCounterPair {
  /// The name of the counter.
  StyleCustomIdent name;
  /// The value of the counter / increment / etc.
  Integer value;

  bool operator==(const StyleGenericCounterPair& other) const {
    return name == other.name &&
           value == other.value;
  }
  bool operator!=(const StyleGenericCounterPair& other) const {
    return name != other.name ||
           value != other.value;
  }
};

/// A generic value for lists of counters.
///
/// Keyword `none` is represented by an empty vector.
template<typename I>
using StyleGenericCounters = StyleOwnedSlice<StyleGenericCounterPair<I>>;

/// A generic value for the `counter-set` and `counter-reset` properties.
template<typename I>
using StyleGenericCounterSetOrReset = StyleGenericCounters<I>;

/// A computed value for the `counter-set` and `counter-reset` properties.
using StyleCounterSetOrReset = StyleGenericCounterSetOrReset<int32_t>;

/// A generic value for the `counter-increment` property.
template<typename I>
using StyleGenericCounterIncrement = StyleGenericCounters<I>;

/// A computed value for the `counter-increment` property.
using StyleCounterIncrement = StyleGenericCounterIncrement<int32_t>;

struct StyleWritingMode {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleWritingMode operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleWritingMode operator|(const StyleWritingMode& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleWritingMode& operator|=(const StyleWritingMode& other) {
    *this = (*this | other);
    return *this;
  }
  StyleWritingMode operator&(const StyleWritingMode& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleWritingMode& operator&=(const StyleWritingMode& other) {
    *this = (*this & other);
    return *this;
  }
  StyleWritingMode operator^(const StyleWritingMode& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleWritingMode& operator^=(const StyleWritingMode& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleWritingMode& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleWritingMode& other) const {
    return bits != other.bits;
  }
  static const StyleWritingMode VERTICAL;
  static const StyleWritingMode INLINE_REVERSED;
  static const StyleWritingMode VERTICAL_LR;
  static const StyleWritingMode LINE_INVERTED;
  static const StyleWritingMode RTL;
  static const StyleWritingMode VERTICAL_SIDEWAYS;
  static const StyleWritingMode TEXT_SIDEWAYS;
  static const StyleWritingMode UPRIGHT;
};
/// A vertical writing mode; writing-mode is vertical-rl,
/// vertical-lr, sideways-lr, or sideways-rl.
inline const StyleWritingMode StyleWritingMode::VERTICAL = StyleWritingMode{ /* .bits = */ (uint8_t)(1 << 0) };
/// The inline flow direction is reversed against the physical
/// direction (i.e. right-to-left or bottom-to-top); writing-mode is
/// sideways-lr or direction is rtl (but not both).
///
/// (This bit can be derived from the others, but we store it for
/// convenience.)
inline const StyleWritingMode StyleWritingMode::INLINE_REVERSED = StyleWritingMode{ /* .bits = */ (uint8_t)(1 << 1) };
/// A vertical writing mode whose block progression direction is left-
/// to-right; writing-mode is vertical-lr or sideways-lr.
///
/// Never set without VERTICAL.
inline const StyleWritingMode StyleWritingMode::VERTICAL_LR = StyleWritingMode{ /* .bits = */ (uint8_t)(1 << 2) };
/// The line-over/line-under sides are inverted with respect to the
/// block-start/block-end edge; writing-mode is vertical-lr.
///
/// Never set without VERTICAL and VERTICAL_LR.
inline const StyleWritingMode StyleWritingMode::LINE_INVERTED = StyleWritingMode{ /* .bits = */ (uint8_t)(1 << 3) };
/// direction is rtl.
inline const StyleWritingMode StyleWritingMode::RTL = StyleWritingMode{ /* .bits = */ (uint8_t)(1 << 4) };
/// All text within a vertical writing mode is displayed sideways
/// and runs top-to-bottom or bottom-to-top; set in these cases:
///
/// * writing-mode: sideways-rl;
/// * writing-mode: sideways-lr;
///
/// Never set without VERTICAL.
inline const StyleWritingMode StyleWritingMode::VERTICAL_SIDEWAYS = StyleWritingMode{ /* .bits = */ (uint8_t)(1 << 5) };
/// Similar to VERTICAL_SIDEWAYS, but is set via text-orientation;
/// set in these cases:
///
/// * writing-mode: vertical-rl; text-orientation: sideways;
/// * writing-mode: vertical-lr; text-orientation: sideways;
///
/// Never set without VERTICAL.
inline const StyleWritingMode StyleWritingMode::TEXT_SIDEWAYS = StyleWritingMode{ /* .bits = */ (uint8_t)(1 << 6) };
/// Horizontal text within a vertical writing mode is displayed with each
/// glyph upright; set in these cases:
///
/// * writing-mode: vertical-rl; text-orientation: upright;
/// * writing-mode: vertical-lr: text-orientation: upright;
///
/// Never set without VERTICAL.
inline const StyleWritingMode StyleWritingMode::UPRIGHT = StyleWritingMode{ /* .bits = */ (uint8_t)(1 << 7) };

/// <https://drafts.csswg.org/css-counter-styles/#typedef-symbol>
union StyleSymbol {
  enum class Tag : uint8_t {
    /// <string>
    String,
    /// <custom-ident>
    Ident,
  };

  struct String_Body {
    Tag tag;
    StyleOwnedStr _0;

    bool operator==(const String_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const String_Body& other) const {
      return _0 != other._0;
    }
  };

  struct Ident_Body {
    Tag tag;
    StyleCustomIdent _0;

    bool operator==(const Ident_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Ident_Body& other) const {
      return _0 != other._0;
    }
  };

  struct {
    Tag tag;
  };
  String_Body string;
  Ident_Body ident;

  static StyleSymbol String(const StyleOwnedStr &_0) {
    StyleSymbol result;
    ::new (&result.string._0) (StyleOwnedStr)(_0);
    result.tag = Tag::String;
    return result;
  }

  bool IsString() const {
    return tag == Tag::String;
  }

  const StyleOwnedStr& AsString() const {
    MOZ_DIAGNOSTIC_ASSERT(IsString());
    return string._0;
  }

  static StyleSymbol Ident(const StyleCustomIdent &_0) {
    StyleSymbol result;
    ::new (&result.ident._0) (StyleCustomIdent)(_0);
    result.tag = Tag::Ident;
    return result;
  }

  bool IsIdent() const {
    return tag == Tag::Ident;
  }

  const StyleCustomIdent& AsIdent() const {
    MOZ_DIAGNOSTIC_ASSERT(IsIdent());
    return ident._0;
  }

  bool operator==(const StyleSymbol& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::String: return string == other.string;
      case Tag::Ident: return ident == other.ident;

    }
    return true;
  }

  bool operator!=(const StyleSymbol& other) const {
    return !(*this == other);
  }

  private:
  StyleSymbol() {

  }
  public:


  ~StyleSymbol() {
    switch (tag) {
      case Tag::String: string.~String_Body(); break;
      case Tag::Ident: ident.~Ident_Body(); break;

    }
  }

  StyleSymbol(const StyleSymbol& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::String: ::new (&string) (String_Body)(other.string); break;
      case Tag::Ident: ::new (&ident) (Ident_Body)(other.ident); break;

    }
  }
  StyleSymbol& operator=(const StyleSymbol& other) {
    if (this != &other) {
      this->~StyleSymbol();
      new (this) StyleSymbol(other);
    }
    return *this;
  }
};

/// <https://drafts.csswg.org/css-counter-styles/#descdef-counter-style-symbols>
struct StyleSymbols {
  StyleOwnedSlice<StyleSymbol> _0;

  bool operator==(const StyleSymbols& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleSymbols& other) const {
    return _0 != other._0;
  }
};

/// <https://drafts.csswg.org/css-counter-styles/#typedef-counter-style>
///
/// Note that 'none' is not a valid name.
union StyleCounterStyle {
  enum class Tag : uint8_t {
    /// `<counter-style-name>`
    Name,
    /// `symbols()`
    Symbols,
  };

  struct Name_Body {
    Tag tag;
    StyleCustomIdent _0;

    bool operator==(const Name_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Name_Body& other) const {
      return _0 != other._0;
    }
  };

  struct Symbols_Body {
    Tag tag;
    StyleSymbolsType _0;
    StyleSymbols _1;

    bool operator==(const Symbols_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const Symbols_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct {
    Tag tag;
  };
  Name_Body name;
  Symbols_Body symbols;

  static StyleCounterStyle Name(const StyleCustomIdent &_0) {
    StyleCounterStyle result;
    ::new (&result.name._0) (StyleCustomIdent)(_0);
    result.tag = Tag::Name;
    return result;
  }

  bool IsName() const {
    return tag == Tag::Name;
  }

  const StyleCustomIdent& AsName() const {
    MOZ_DIAGNOSTIC_ASSERT(IsName());
    return name._0;
  }

  static StyleCounterStyle Symbols(const StyleSymbolsType &_0,
                                   const StyleSymbols &_1) {
    StyleCounterStyle result;
    ::new (&result.symbols._0) (StyleSymbolsType)(_0);
    ::new (&result.symbols._1) (StyleSymbols)(_1);
    result.tag = Tag::Symbols;
    return result;
  }

  bool IsSymbols() const {
    return tag == Tag::Symbols;
  }

  const Symbols_Body& AsSymbols() const {
    MOZ_DIAGNOSTIC_ASSERT(IsSymbols());
    return symbols;
  }

  bool operator==(const StyleCounterStyle& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Name: return name == other.name;
      case Tag::Symbols: return symbols == other.symbols;

    }
    return true;
  }

  bool operator!=(const StyleCounterStyle& other) const {
    return !(*this == other);
  }

  private:
  StyleCounterStyle() {

  }
  public:


  ~StyleCounterStyle() {
    switch (tag) {
      case Tag::Name: name.~Name_Body(); break;
      case Tag::Symbols: symbols.~Symbols_Body(); break;

    }
  }

  StyleCounterStyle(const StyleCounterStyle& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Name: ::new (&name) (Name_Body)(other.name); break;
      case Tag::Symbols: ::new (&symbols) (Symbols_Body)(other.symbols); break;

    }
  }
  StyleCounterStyle& operator=(const StyleCounterStyle& other) {
    if (this != &other) {
      this->~StyleCounterStyle();
      new (this) StyleCounterStyle(other);
    }
    return *this;
  }
};

#if defined(CBINDGEN_IS_SERVO)
using StyleCounterStyleType = StyleListStyleType;
#endif

#if defined(CBINDGEN_IS_GECKO)
using StyleCounterStyleType = StyleCounterStyle;
#endif

/// A CSS `<ident>` stored as an `Atom`.
using StyleAtomIdent = StyleAtom;

#if defined(CBINDGEN_IS_GECKO)
/// The namespace prefix type for Gecko, which is just an atom.
using StylePrefix = StyleAtomIdent;
#endif

#if defined(CBINDGEN_IS_GECKO)
/// A Gecko namespace is just a wrapped atom.
using StyleNamespace = StyleAtom;
#endif

/// An attr(...) rule
///
/// `[namespace? `|`]? ident`
struct StyleAttr {
  /// Optional namespace prefix.
  StylePrefix namespace_prefix;
  /// Optional namespace URL.
  StyleNamespace namespace_url;
  /// Attribute name
  StyleAtom attribute;

  bool operator==(const StyleAttr& other) const {
    return namespace_prefix == other.namespace_prefix &&
           namespace_url == other.namespace_url &&
           attribute == other.attribute;
  }
  bool operator!=(const StyleAttr& other) const {
    return namespace_prefix != other.namespace_prefix ||
           namespace_url != other.namespace_url ||
           attribute != other.attribute;
  }
};

/// Items for the `content` property.
template<typename ImageUrl>
union StyleGenericContentItem {
  enum class Tag : uint8_t {
    /// Literal string content.
    String,
    /// `counter(name, style)`.
    Counter,
    /// `counters(name, separator, style)`.
    Counters,
    /// `open-quote`.
    OpenQuote,
    /// `close-quote`.
    CloseQuote,
    /// `no-open-quote`.
    NoOpenQuote,
    /// `no-close-quote`.
    NoCloseQuote,
#if defined(CBINDGEN_IS_GECKO)
    /// `-moz-alt-content`.
    MozAltContent,
#endif
#if (defined(CBINDGEN_IS_GECKO) || defined(CBINDGEN_IS_SERVO))
    /// `attr([namespace? `|`]? ident)`
    Attr,
#endif
    /// `url(url)`
    Url,
  };

  struct String_Body {
    Tag tag;
    StyleOwnedStr _0;

    bool operator==(const String_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const String_Body& other) const {
      return _0 != other._0;
    }
  };

  struct Counter_Body {
    Tag tag;
    StyleCustomIdent _0;
    StyleCounterStyleType _1;

    bool operator==(const Counter_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const Counter_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct Counters_Body {
    Tag tag;
    StyleCustomIdent _0;
    StyleOwnedStr _1;
    StyleCounterStyleType _2;

    bool operator==(const Counters_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1 &&
             _2 == other._2;
    }
    bool operator!=(const Counters_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1 ||
             _2 != other._2;
    }
  };

#if (defined(CBINDGEN_IS_GECKO) || defined(CBINDGEN_IS_SERVO))
  struct Attr_Body {
    Tag tag;
    StyleAttr _0;

    bool operator==(const Attr_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Attr_Body& other) const {
      return _0 != other._0;
    }
  };
#endif

  struct Url_Body {
    Tag tag;
    ImageUrl _0;

    bool operator==(const Url_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Url_Body& other) const {
      return _0 != other._0;
    }
  };

  struct {
    Tag tag;
  };
  String_Body string;
  Counter_Body counter;
  Counters_Body counters;
#if (defined(CBINDGEN_IS_GECKO) || defined(CBINDGEN_IS_SERVO))
  Attr_Body attr;
#endif
  Url_Body url;

  static StyleGenericContentItem String(const StyleOwnedStr &_0) {
    StyleGenericContentItem result;
    ::new (&result.string._0) (StyleOwnedStr)(_0);
    result.tag = Tag::String;
    return result;
  }

  bool IsString() const {
    return tag == Tag::String;
  }

  const StyleOwnedStr& AsString() const {
    MOZ_DIAGNOSTIC_ASSERT(IsString());
    return string._0;
  }

  static StyleGenericContentItem Counter(const StyleCustomIdent &_0,
                                         const StyleCounterStyleType &_1) {
    StyleGenericContentItem result;
    ::new (&result.counter._0) (StyleCustomIdent)(_0);
    ::new (&result.counter._1) (StyleCounterStyleType)(_1);
    result.tag = Tag::Counter;
    return result;
  }

  bool IsCounter() const {
    return tag == Tag::Counter;
  }

  const Counter_Body& AsCounter() const {
    MOZ_DIAGNOSTIC_ASSERT(IsCounter());
    return counter;
  }

  static StyleGenericContentItem Counters(const StyleCustomIdent &_0,
                                          const StyleOwnedStr &_1,
                                          const StyleCounterStyleType &_2) {
    StyleGenericContentItem result;
    ::new (&result.counters._0) (StyleCustomIdent)(_0);
    ::new (&result.counters._1) (StyleOwnedStr)(_1);
    ::new (&result.counters._2) (StyleCounterStyleType)(_2);
    result.tag = Tag::Counters;
    return result;
  }

  bool IsCounters() const {
    return tag == Tag::Counters;
  }

  const Counters_Body& AsCounters() const {
    MOZ_DIAGNOSTIC_ASSERT(IsCounters());
    return counters;
  }

  static StyleGenericContentItem OpenQuote() {
    StyleGenericContentItem result;
    result.tag = Tag::OpenQuote;
    return result;
  }

  bool IsOpenQuote() const {
    return tag == Tag::OpenQuote;
  }

  static StyleGenericContentItem CloseQuote() {
    StyleGenericContentItem result;
    result.tag = Tag::CloseQuote;
    return result;
  }

  bool IsCloseQuote() const {
    return tag == Tag::CloseQuote;
  }

  static StyleGenericContentItem NoOpenQuote() {
    StyleGenericContentItem result;
    result.tag = Tag::NoOpenQuote;
    return result;
  }

  bool IsNoOpenQuote() const {
    return tag == Tag::NoOpenQuote;
  }

  static StyleGenericContentItem NoCloseQuote() {
    StyleGenericContentItem result;
    result.tag = Tag::NoCloseQuote;
    return result;
  }

  bool IsNoCloseQuote() const {
    return tag == Tag::NoCloseQuote;
  }

#if defined(CBINDGEN_IS_GECKO)
  static StyleGenericContentItem MozAltContent() {
    StyleGenericContentItem result;
    result.tag = Tag::MozAltContent;
    return result;
  }

  bool IsMozAltContent() const {
    return tag == Tag::MozAltContent;
  }
#endif

#if (defined(CBINDGEN_IS_GECKO) || defined(CBINDGEN_IS_SERVO))
  static StyleGenericContentItem Attr(const StyleAttr &_0) {
    StyleGenericContentItem result;
    ::new (&result.attr._0) (StyleAttr)(_0);
    result.tag = Tag::Attr;
    return result;
  }

  bool IsAttr() const {
    return tag == Tag::Attr;
  }

  const StyleAttr& AsAttr() const {
    MOZ_DIAGNOSTIC_ASSERT(IsAttr());
    return attr._0;
  }
#endif

  static StyleGenericContentItem Url(const ImageUrl &_0) {
    StyleGenericContentItem result;
    ::new (&result.url._0) (ImageUrl)(_0);
    result.tag = Tag::Url;
    return result;
  }

  bool IsUrl() const {
    return tag == Tag::Url;
  }

  const ImageUrl& AsUrl() const {
    MOZ_DIAGNOSTIC_ASSERT(IsUrl());
    return url._0;
  }

  bool operator==(const StyleGenericContentItem& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::String: return string == other.string;
      case Tag::Counter: return counter == other.counter;
      case Tag::Counters: return counters == other.counters;
#if (defined(CBINDGEN_IS_GECKO) || defined(CBINDGEN_IS_SERVO))
      case Tag::Attr: return attr == other.attr;
#endif
      case Tag::Url: return url == other.url;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericContentItem& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericContentItem() {

  }
  public:


  ~StyleGenericContentItem() {
    switch (tag) {
      case Tag::String: string.~String_Body(); break;
      case Tag::Counter: counter.~Counter_Body(); break;
      case Tag::Counters: counters.~Counters_Body(); break;
#if (defined(CBINDGEN_IS_GECKO) || defined(CBINDGEN_IS_SERVO))
      case Tag::Attr: attr.~Attr_Body(); break;
#endif
      case Tag::Url: url.~Url_Body(); break;
      default: break;
    }
  }

  StyleGenericContentItem(const StyleGenericContentItem& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::String: ::new (&string) (String_Body)(other.string); break;
      case Tag::Counter: ::new (&counter) (Counter_Body)(other.counter); break;
      case Tag::Counters: ::new (&counters) (Counters_Body)(other.counters); break;
#if (defined(CBINDGEN_IS_GECKO) || defined(CBINDGEN_IS_SERVO))
      case Tag::Attr: ::new (&attr) (Attr_Body)(other.attr); break;
#endif
      case Tag::Url: ::new (&url) (Url_Body)(other.url); break;
      default: break;
    }
  }
  StyleGenericContentItem& operator=(const StyleGenericContentItem& other) {
    if (this != &other) {
      this->~StyleGenericContentItem();
      new (this) StyleGenericContentItem(other);
    }
    return *this;
  }
};

/// The specified value for the `content` property.
///
/// https://drafts.csswg.org/css-content/#propdef-content
template<typename ImageUrl>
union StyleGenericContent {
  enum class Tag : uint8_t {
    /// `normal` reserved keyword.
    Normal,
    /// `none` reserved keyword.
    None,
    /// Content items.
    Items,
  };

  struct Items_Body {
    Tag tag;
    StyleOwnedSlice<StyleGenericContentItem<ImageUrl>> _0;

    bool operator==(const Items_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Items_Body& other) const {
      return _0 != other._0;
    }
  };

  struct {
    Tag tag;
  };
  Items_Body items;

  static StyleGenericContent Normal() {
    StyleGenericContent result;
    result.tag = Tag::Normal;
    return result;
  }

  bool IsNormal() const {
    return tag == Tag::Normal;
  }

  static StyleGenericContent None() {
    StyleGenericContent result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericContent Items(const StyleOwnedSlice<StyleGenericContentItem<ImageUrl>> &_0) {
    StyleGenericContent result;
    ::new (&result.items._0) (StyleOwnedSlice<StyleGenericContentItem<ImageUrl>>)(_0);
    result.tag = Tag::Items;
    return result;
  }

  bool IsItems() const {
    return tag == Tag::Items;
  }

  const StyleOwnedSlice<StyleGenericContentItem<ImageUrl>>& AsItems() const {
    MOZ_DIAGNOSTIC_ASSERT(IsItems());
    return items._0;
  }

  bool operator==(const StyleGenericContent& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Items: return items == other.items;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericContent& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericContent() {

  }
  public:


  ~StyleGenericContent() {
    switch (tag) {
      case Tag::Items: items.~Items_Body(); break;
      default: break;
    }
  }

  StyleGenericContent(const StyleGenericContent& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Items: ::new (&items) (Items_Body)(other.items); break;
      default: break;
    }
  }
  StyleGenericContent& operator=(const StyleGenericContent& other) {
    if (this != &other) {
      this->~StyleGenericContent();
      new (this) StyleGenericContent(other);
    }
    return *this;
  }
};

/// A computed value for the `content` property.
using StyleContent = StyleGenericContent<StyleComputedImageUrl>;

/// A computed content item.
using StyleContentItem = StyleGenericContentItem<StyleComputedImageUrl>;

#if defined(CBINDGEN_IS_GECKO)
/// Constants shared by multiple CSS Box Alignment properties
struct StyleAlignFlags {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleAlignFlags operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleAlignFlags operator|(const StyleAlignFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleAlignFlags& operator|=(const StyleAlignFlags& other) {
    *this = (*this | other);
    return *this;
  }
  StyleAlignFlags operator&(const StyleAlignFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleAlignFlags& operator&=(const StyleAlignFlags& other) {
    *this = (*this & other);
    return *this;
  }
  StyleAlignFlags operator^(const StyleAlignFlags& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleAlignFlags& operator^=(const StyleAlignFlags& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleAlignFlags& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleAlignFlags& other) const {
    return bits != other.bits;
  }
  static const StyleAlignFlags AUTO;
  static const StyleAlignFlags NORMAL;
  static const StyleAlignFlags START;
  static const StyleAlignFlags END;
  static const StyleAlignFlags FLEX_START;
  static const StyleAlignFlags FLEX_END;
  static const StyleAlignFlags CENTER;
  static const StyleAlignFlags LEFT;
  static const StyleAlignFlags RIGHT;
  static const StyleAlignFlags BASELINE;
  static const StyleAlignFlags LAST_BASELINE;
  static const StyleAlignFlags STRETCH;
  static const StyleAlignFlags SELF_START;
  static const StyleAlignFlags SELF_END;
  static const StyleAlignFlags SPACE_BETWEEN;
  static const StyleAlignFlags SPACE_AROUND;
  static const StyleAlignFlags SPACE_EVENLY;
  static const StyleAlignFlags LEGACY;
  static const StyleAlignFlags SAFE;
  static const StyleAlignFlags UNSAFE;
  static const StyleAlignFlags FLAG_BITS;
};
#if defined(CBINDGEN_IS_GECKO)
/// {align,justify}-{content,items,self}: 'auto'
inline const StyleAlignFlags StyleAlignFlags::AUTO = StyleAlignFlags{ /* .bits = */ (uint8_t)0 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'normal'
inline const StyleAlignFlags StyleAlignFlags::NORMAL = StyleAlignFlags{ /* .bits = */ (uint8_t)1 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'start'
inline const StyleAlignFlags StyleAlignFlags::START = StyleAlignFlags{ /* .bits = */ (uint8_t)2 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'end'
inline const StyleAlignFlags StyleAlignFlags::END = StyleAlignFlags{ /* .bits = */ (uint8_t)3 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'flex-start'
inline const StyleAlignFlags StyleAlignFlags::FLEX_START = StyleAlignFlags{ /* .bits = */ (uint8_t)4 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'flex-end'
inline const StyleAlignFlags StyleAlignFlags::FLEX_END = StyleAlignFlags{ /* .bits = */ (uint8_t)5 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'center'
inline const StyleAlignFlags StyleAlignFlags::CENTER = StyleAlignFlags{ /* .bits = */ (uint8_t)6 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'left'
inline const StyleAlignFlags StyleAlignFlags::LEFT = StyleAlignFlags{ /* .bits = */ (uint8_t)7 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'right'
inline const StyleAlignFlags StyleAlignFlags::RIGHT = StyleAlignFlags{ /* .bits = */ (uint8_t)8 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'baseline'
inline const StyleAlignFlags StyleAlignFlags::BASELINE = StyleAlignFlags{ /* .bits = */ (uint8_t)9 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'last-baseline'
inline const StyleAlignFlags StyleAlignFlags::LAST_BASELINE = StyleAlignFlags{ /* .bits = */ (uint8_t)10 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'stretch'
inline const StyleAlignFlags StyleAlignFlags::STRETCH = StyleAlignFlags{ /* .bits = */ (uint8_t)11 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'self-start'
inline const StyleAlignFlags StyleAlignFlags::SELF_START = StyleAlignFlags{ /* .bits = */ (uint8_t)12 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'self-end'
inline const StyleAlignFlags StyleAlignFlags::SELF_END = StyleAlignFlags{ /* .bits = */ (uint8_t)13 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'space-between'
inline const StyleAlignFlags StyleAlignFlags::SPACE_BETWEEN = StyleAlignFlags{ /* .bits = */ (uint8_t)14 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'space-around'
inline const StyleAlignFlags StyleAlignFlags::SPACE_AROUND = StyleAlignFlags{ /* .bits = */ (uint8_t)15 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'space-evenly'
inline const StyleAlignFlags StyleAlignFlags::SPACE_EVENLY = StyleAlignFlags{ /* .bits = */ (uint8_t)16 };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'legacy' (mutually exclusive w. SAFE & UNSAFE)
inline const StyleAlignFlags StyleAlignFlags::LEGACY = StyleAlignFlags{ /* .bits = */ (uint8_t)(1 << 5) };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'safe'
inline const StyleAlignFlags StyleAlignFlags::SAFE = StyleAlignFlags{ /* .bits = */ (uint8_t)(1 << 6) };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// 'unsafe' (mutually exclusive w. SAFE)
inline const StyleAlignFlags StyleAlignFlags::UNSAFE = StyleAlignFlags{ /* .bits = */ (uint8_t)(1 << 7) };
#endif
#if defined(CBINDGEN_IS_GECKO)
/// Mask for the additional flags above.
inline const StyleAlignFlags StyleAlignFlags::FLAG_BITS = StyleAlignFlags{ /* .bits = */ (uint8_t)224 };
#endif
#endif

#if defined(CBINDGEN_IS_GECKO)
/// <https://drafts.csswg.org/css-align/#self-alignment>
using StyleSelfAlignment = StyleAlignFlags;
#endif

#if defined(CBINDGEN_IS_GECKO)
/// The specified value of the align-self property.
///
/// <https://drafts.csswg.org/css-align/#propdef-align-self>
struct StyleAlignSelf {
  StyleSelfAlignment _0;

  bool operator==(const StyleAlignSelf& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleAlignSelf& other) const {
    return _0 != other._0;
  }
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// The specified value of the justify-self property.
///
/// <https://drafts.csswg.org/css-align/#propdef-justify-self>
struct StyleJustifySelf {
  StyleSelfAlignment _0;

  bool operator==(const StyleJustifySelf& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleJustifySelf& other) const {
    return _0 != other._0;
  }
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// Value of the `align-items` property
///
/// <https://drafts.csswg.org/css-align/#propdef-align-items>
struct StyleAlignItems {
  StyleAlignFlags _0;

  bool operator==(const StyleAlignItems& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleAlignItems& other) const {
    return _0 != other._0;
  }
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// Value of the `justify-items` property
///
/// <https://drafts.csswg.org/css-align/#justify-items-property>
struct StyleJustifyItems {
  StyleAlignFlags _0;

  bool operator==(const StyleJustifyItems& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleJustifyItems& other) const {
    return _0 != other._0;
  }
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// The computed value for the `justify-items` property.
///
/// Need to carry around both the specified and computed value to handle the
/// special legacy keyword without destroying style sharing.
///
/// In particular, `justify-items` is a reset property, so we ought to be able
/// to share its computed representation across elements as long as they match
/// the same rules. Except that it's not true if the specified value for
/// `justify-items` is `legacy` and the computed value of the parent has the
/// `legacy` modifier.
///
/// So instead of computing `legacy` "normally" looking at get_parent_position(),
/// marking it as uncacheable, we carry the specified value around and handle
/// the special case in `StyleAdjuster` instead, only when the result of the
/// computation would vary.
///
/// Note that we also need to special-case this property in matching.rs, in
/// order to properly handle changes to the legacy keyword... This all kinda
/// sucks :(.
///
/// See the discussion in https://bugzil.la/1384542.
struct StyleComputedJustifyItems {
  /// The specified value for the property. Can contain the bare `legacy`
  /// keyword.
  StyleJustifyItems specified;
  /// The computed value for the property. Cannot contain the bare `legacy`
  /// keyword, but note that it could contain it in combination with other
  /// keywords like `left`, `right` or `center`.
  StyleJustifyItems computed;

  bool operator==(const StyleComputedJustifyItems& other) const {
    return specified == other.specified &&
           computed == other.computed;
  }
  bool operator!=(const StyleComputedJustifyItems& other) const {
    return specified != other.specified ||
           computed != other.computed;
  }
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// Shared value for the `align-content` and `justify-content` properties.
///
/// <https://drafts.csswg.org/css-align/#content-distribution>
struct StyleContentDistribution {
  StyleAlignFlags primary;

  bool operator==(const StyleContentDistribution& other) const {
    return primary == other.primary;
  }
  bool operator!=(const StyleContentDistribution& other) const {
    return primary != other.primary;
  }
};
#endif

#if defined(CBINDGEN_IS_GECKO)
/// Value for the `align-content` property.
///
/// <https://drafts.csswg.org/css-align/#propdef-align-content>
using StyleAlignContent = StyleContentDistribution;
#endif

#if defined(CBINDGEN_IS_GECKO)
/// Value for the `justify-content` property.
///
/// <https://drafts.csswg.org/css-align/#propdef-justify-content>
using StyleJustifyContent = StyleContentDistribution;
#endif

#if defined(CBINDGEN_IS_GECKO)
/// Value for the `align-tracks` property.
///
/// <https://github.com/w3c/csswg-drafts/issues/4650>
using StyleAlignTracks = StyleOwnedSlice<StyleAlignContent>;
#endif

#if defined(CBINDGEN_IS_GECKO)
/// Value for the `justify-tracks` property.
///
/// <https://github.com/w3c/csswg-drafts/issues/4650>
using StyleJustifyTracks = StyleOwnedSlice<StyleJustifyContent>;
#endif

/// Values for `moz-image-rect`.
///
/// `-moz-image-rect(<uri>, top, right, bottom, left);`
template<typename NumberOrPercentage, typename MozImageRectUrl>
struct StyleGenericMozImageRect {
  MozImageRectUrl url;
  NumberOrPercentage top;
  NumberOrPercentage right;
  NumberOrPercentage bottom;
  NumberOrPercentage left;

  bool operator==(const StyleGenericMozImageRect& other) const {
    return url == other.url &&
           top == other.top &&
           right == other.right &&
           bottom == other.bottom &&
           left == other.left;
  }
  bool operator!=(const StyleGenericMozImageRect& other) const {
    return url != other.url ||
           top != other.top ||
           right != other.right ||
           bottom != other.bottom ||
           left != other.left;
  }
};

#if defined(CBINDGEN_IS_SERVO)
/// A specified image url() value for servo.
using StyleSpecifiedImageUrl = StyleCssUrl;
#endif

#if defined(CBINDGEN_IS_GECKO)
/// Computed values for `-moz-image-rect(...)`.
using StyleMozImageRect = StyleGenericMozImageRect<StyleNumberOrPercentage, StyleComputedImageUrl>;
#endif

#if !defined(CBINDGEN_IS_GECKO)
/// Empty enum on non-gecko
using StyleMozImageRect = StyleMozImageRect;
#endif

#if (defined(CBINDGEN_IS_GECKO) && !defined(CBINDGEN_IS_GECKO))
/// Specified values for `moz-image-rect`
/// -moz-image-rect(<uri>, top, right, bottom, left);
using StyleMozImageRect = StyleGenericMozImageRect<StyleNumberOrPercentage, StyleSpecifiedImageUrl>;
#endif

/// A computed `<resolution>`.
struct StyleResolution {
  StyleCSSFloat _0;

  bool operator==(const StyleResolution& other) const {
    return _0 == other._0;
  }
  bool operator!=(const StyleResolution& other) const {
    return _0 != other._0;
  }
};

/// A `<percent> | none` value. Represents optional percentage values
/// assosicated with cross-fade images.
template<typename Percentage>
struct StylePercentOrNone {
  enum class Tag : uint8_t {
    /// `none` variant.
    None,
    /// A percentage variant.
    Percent,
  };

  struct StylePercent_Body {
    Percentage _0;

    bool operator==(const StylePercent_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StylePercent_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StylePercent_Body percent;
  };

  static StylePercentOrNone None() {
    StylePercentOrNone result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StylePercentOrNone Percent(const Percentage &_0) {
    StylePercentOrNone result;
    ::new (&result.percent._0) (Percentage)(_0);
    result.tag = Tag::Percent;
    return result;
  }

  bool IsPercent() const {
    return tag == Tag::Percent;
  }

  const Percentage& AsPercent() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPercent());
    return percent._0;
  }

  bool operator==(const StylePercentOrNone& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Percent: return percent == other.percent;
      default: break;
    }
    return true;
  }

  bool operator!=(const StylePercentOrNone& other) const {
    return !(*this == other);
  }

  private:
  StylePercentOrNone() {

  }
  public:


  ~StylePercentOrNone() {
    switch (tag) {
      case Tag::Percent: percent.~StylePercent_Body(); break;
      default: break;
    }
  }

  StylePercentOrNone(const StylePercentOrNone& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Percent: ::new (&percent) (StylePercent_Body)(other.percent); break;
      default: break;
    }
  }
  StylePercentOrNone& operator=(const StylePercentOrNone& other) {
    if (this != &other) {
      this->~StylePercentOrNone();
      new (this) StylePercentOrNone(other);
    }
    return *this;
  }
};

/// An image or a color. `cross-fade` takes either when blending
/// images together.
template<typename I, typename C>
struct StyleGenericCrossFadeImage {
  enum class Tag : uint8_t {
    /// A boxed image value. Boxing provides indirection so images can
    /// be cross-fades and cross-fades can be images.
    Image,
    /// A color value.
    Color,
  };

  struct StyleImage_Body {
    I _0;

    bool operator==(const StyleImage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleImage_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleColor_Body {
    C _0;

    bool operator==(const StyleColor_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleColor_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleImage_Body image;
    StyleColor_Body color;
  };

  static StyleGenericCrossFadeImage Image(const I &_0) {
    StyleGenericCrossFadeImage result;
    ::new (&result.image._0) (I)(_0);
    result.tag = Tag::Image;
    return result;
  }

  bool IsImage() const {
    return tag == Tag::Image;
  }

  const I& AsImage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsImage());
    return image._0;
  }

  static StyleGenericCrossFadeImage Color(const C &_0) {
    StyleGenericCrossFadeImage result;
    ::new (&result.color._0) (C)(_0);
    result.tag = Tag::Color;
    return result;
  }

  bool IsColor() const {
    return tag == Tag::Color;
  }

  const C& AsColor() const {
    MOZ_DIAGNOSTIC_ASSERT(IsColor());
    return color._0;
  }

  bool operator==(const StyleGenericCrossFadeImage& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Image: return image == other.image;
      case Tag::Color: return color == other.color;

    }
    return true;
  }

  bool operator!=(const StyleGenericCrossFadeImage& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericCrossFadeImage() {

  }
  public:


  ~StyleGenericCrossFadeImage() {
    switch (tag) {
      case Tag::Image: image.~StyleImage_Body(); break;
      case Tag::Color: color.~StyleColor_Body(); break;

    }
  }

  StyleGenericCrossFadeImage(const StyleGenericCrossFadeImage& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Image: ::new (&image) (StyleImage_Body)(other.image); break;
      case Tag::Color: ::new (&color) (StyleColor_Body)(other.color); break;

    }
  }
  StyleGenericCrossFadeImage& operator=(const StyleGenericCrossFadeImage& other) {
    if (this != &other) {
      this->~StyleGenericCrossFadeImage();
      new (this) StyleGenericCrossFadeImage(other);
    }
    return *this;
  }
};

/// An optional percent and a cross fade image.
template<typename Image, typename Color, typename Percentage>
struct StyleGenericCrossFadeElement {
  /// The percent of the final image that `image` will be.
  StylePercentOrNone<Percentage> percent;
  /// A color or image that will be blended when cross-fade is
  /// evaluated.
  StyleGenericCrossFadeImage<Image, Color> image;

  bool operator==(const StyleGenericCrossFadeElement& other) const {
    return percent == other.percent &&
           image == other.image;
  }
  bool operator!=(const StyleGenericCrossFadeElement& other) const {
    return percent != other.percent ||
           image != other.image;
  }
};

/// <https://drafts.csswg.org/css-images-4/#cross-fade-function>
template<typename Image, typename Color, typename Percentage>
struct StyleGenericCrossFade {
  /// All of the image percent pairings passed as arguments to
  /// cross-fade.
  StyleOwnedSlice<StyleGenericCrossFadeElement<Image, Color, Percentage>> elements;

  bool operator==(const StyleGenericCrossFade& other) const {
    return elements == other.elements;
  }
  bool operator!=(const StyleGenericCrossFade& other) const {
    return elements != other.elements;
  }
};

/// An optional percent and a cross fade image.
template<typename Image, typename Resolution>
struct StyleGenericImageSetItem {
  /// `<image>`. `<string>` is converted to `Image::Url` at parse time.
  Image image;
  /// The `<resolution>`.
  ///
  /// TODO: Skip serialization if it is 1x.
  Resolution resolution;

  bool operator==(const StyleGenericImageSetItem& other) const {
    return image == other.image &&
           resolution == other.resolution;
  }
  bool operator!=(const StyleGenericImageSetItem& other) const {
    return image != other.image ||
           resolution != other.resolution;
  }
};

/// https://drafts.csswg.org/css-images-4/#image-set-notation
template<typename Image, typename Resolution>
struct StyleGenericImageSet {
  /// The index of the selected candidate. Zero for specified values.
  uintptr_t selected_index;
  /// All of the image and resolution pairs.
  StyleOwnedSlice<StyleGenericImageSetItem<Image, Resolution>> items;

  bool operator==(const StyleGenericImageSet& other) const {
    return selected_index == other.selected_index &&
           items == other.items;
  }
  bool operator!=(const StyleGenericImageSet& other) const {
    return selected_index != other.selected_index ||
           items != other.items;
  }
};

/// An `<image> | none` value.
///
/// https://drafts.csswg.org/css-images/#image-values
template<typename G, typename MozImageRect, typename ImageUrl, typename Color, typename Percentage, typename Resolution>
struct StyleGenericImage {
  enum class Tag : uint8_t {
    /// `none` variant.
    None,
    /// A `<url()>` image.
    Url,
    /// A `<gradient>` image.  Gradients are rather large, and not nearly as
    /// common as urls, so we box them here to keep the size of this enum sane.
    Gradient,
    /// A `-moz-image-rect` image.  Also fairly large and rare.
    Rect,
#if defined(CBINDGEN_IS_GECKO)
    /// A `-moz-element(# <element-id>)`
    Element,
#endif
#if defined(CBINDGEN_IS_SERVO)
    /// A paint worklet image.
    /// <https://drafts.css-houdini.org/css-paint-api/>
    PaintWorklet,
#endif
    /// A `<cross-fade()>` image. Storing this directly inside of
    /// GenericImage increases the size by 8 bytes so we box it here
    /// and store images directly inside of cross-fade instead of
    /// boxing them there.
    CrossFade,
    /// An `image-set()` function.
    ImageSet,
  };

  struct StyleUrl_Body {
    ImageUrl _0;

    bool operator==(const StyleUrl_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleUrl_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleGradient_Body {
    StyleBox<G> _0;

    bool operator==(const StyleGradient_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleGradient_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleRect_Body {
    StyleBox<MozImageRect> _0;

    bool operator==(const StyleRect_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleRect_Body& other) const {
      return _0 != other._0;
    }
  };

#if defined(CBINDGEN_IS_GECKO)
  struct StyleElement_Body {
    StyleAtom _0;

    bool operator==(const StyleElement_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleElement_Body& other) const {
      return _0 != other._0;
    }
  };
#endif

#if defined(CBINDGEN_IS_SERVO)
  struct StylePaintWorklet_Body {
    StylePaintWorklet _0;

    bool operator==(const StylePaintWorklet_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StylePaintWorklet_Body& other) const {
      return _0 != other._0;
    }
  };
#endif

  struct StyleCrossFade_Body {
    StyleBox<StyleGenericCrossFade<StyleGenericImage, Color, Percentage>> _0;

    bool operator==(const StyleCrossFade_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleCrossFade_Body& other) const {
      return _0 != other._0;
    }
  };

  struct StyleImageSet_Body {
    StyleBox<StyleGenericImageSet<StyleGenericImage, Resolution>> _0;

    bool operator==(const StyleImageSet_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleImageSet_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleUrl_Body url;
    StyleGradient_Body gradient;
    StyleRect_Body rect;
#if defined(CBINDGEN_IS_GECKO)
    StyleElement_Body element;
#endif
#if defined(CBINDGEN_IS_SERVO)
    StylePaintWorklet_Body paint_worklet;
#endif
    StyleCrossFade_Body cross_fade;
    StyleImageSet_Body image_set;
  };

  static StyleGenericImage None() {
    StyleGenericImage result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericImage Url(const ImageUrl &_0) {
    StyleGenericImage result;
    ::new (&result.url._0) (ImageUrl)(_0);
    result.tag = Tag::Url;
    return result;
  }

  bool IsUrl() const {
    return tag == Tag::Url;
  }

  const ImageUrl& AsUrl() const {
    MOZ_DIAGNOSTIC_ASSERT(IsUrl());
    return url._0;
  }

  static StyleGenericImage Gradient(const StyleBox<G> &_0) {
    StyleGenericImage result;
    ::new (&result.gradient._0) (StyleBox<G>)(_0);
    result.tag = Tag::Gradient;
    return result;
  }

  bool IsGradient() const {
    return tag == Tag::Gradient;
  }

  const StyleBox<G>& AsGradient() const {
    MOZ_DIAGNOSTIC_ASSERT(IsGradient());
    return gradient._0;
  }

  static StyleGenericImage Rect(const StyleBox<MozImageRect> &_0) {
    StyleGenericImage result;
    ::new (&result.rect._0) (StyleBox<MozImageRect>)(_0);
    result.tag = Tag::Rect;
    return result;
  }

  bool IsRect() const {
    return tag == Tag::Rect;
  }

  const StyleBox<MozImageRect>& AsRect() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRect());
    return rect._0;
  }

#if defined(CBINDGEN_IS_GECKO)
  static StyleGenericImage Element(const StyleAtom &_0) {
    StyleGenericImage result;
    ::new (&result.element._0) (StyleAtom)(_0);
    result.tag = Tag::Element;
    return result;
  }

  bool IsElement() const {
    return tag == Tag::Element;
  }

  const StyleAtom& AsElement() const {
    MOZ_DIAGNOSTIC_ASSERT(IsElement());
    return element._0;
  }
#endif

#if defined(CBINDGEN_IS_SERVO)
  static StyleGenericImage PaintWorklet(const StylePaintWorklet &_0) {
    StyleGenericImage result;
    ::new (&result.paint_worklet._0) (StylePaintWorklet)(_0);
    result.tag = Tag::PaintWorklet;
    return result;
  }

  bool IsPaintWorklet() const {
    return tag == Tag::PaintWorklet;
  }

  const StylePaintWorklet& AsPaintWorklet() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPaintWorklet());
    return paint_worklet._0;
  }
#endif

  static StyleGenericImage CrossFade(const StyleBox<StyleGenericCrossFade<StyleGenericImage, Color, Percentage>> &_0) {
    StyleGenericImage result;
    ::new (&result.cross_fade._0) (StyleBox<StyleGenericCrossFade<StyleGenericImage, Color, Percentage>>)(_0);
    result.tag = Tag::CrossFade;
    return result;
  }

  bool IsCrossFade() const {
    return tag == Tag::CrossFade;
  }

  const StyleBox<StyleGenericCrossFade<StyleGenericImage, Color, Percentage>>& AsCrossFade() const {
    MOZ_DIAGNOSTIC_ASSERT(IsCrossFade());
    return cross_fade._0;
  }

  static StyleGenericImage ImageSet(const StyleBox<StyleGenericImageSet<StyleGenericImage, Resolution>> &_0) {
    StyleGenericImage result;
    ::new (&result.image_set._0) (StyleBox<StyleGenericImageSet<StyleGenericImage, Resolution>>)(_0);
    result.tag = Tag::ImageSet;
    return result;
  }

  bool IsImageSet() const {
    return tag == Tag::ImageSet;
  }

  const StyleBox<StyleGenericImageSet<StyleGenericImage, Resolution>>& AsImageSet() const {
    MOZ_DIAGNOSTIC_ASSERT(IsImageSet());
    return image_set._0;
  }

  bool operator==(const StyleGenericImage& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Url: return url == other.url;
      case Tag::Gradient: return gradient == other.gradient;
      case Tag::Rect: return rect == other.rect;
#if defined(CBINDGEN_IS_GECKO)
      case Tag::Element: return element == other.element;
#endif
#if defined(CBINDGEN_IS_SERVO)
      case Tag::PaintWorklet: return paint_worklet == other.paint_worklet;
#endif
      case Tag::CrossFade: return cross_fade == other.cross_fade;
      case Tag::ImageSet: return image_set == other.image_set;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericImage& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericImage() {

  }
  public:


  ~StyleGenericImage() {
    switch (tag) {
      case Tag::Url: url.~StyleUrl_Body(); break;
      case Tag::Gradient: gradient.~StyleGradient_Body(); break;
      case Tag::Rect: rect.~StyleRect_Body(); break;
#if defined(CBINDGEN_IS_GECKO)
      case Tag::Element: element.~StyleElement_Body(); break;
#endif
#if defined(CBINDGEN_IS_SERVO)
      case Tag::PaintWorklet: paint_worklet.~StylePaintWorklet_Body(); break;
#endif
      case Tag::CrossFade: cross_fade.~StyleCrossFade_Body(); break;
      case Tag::ImageSet: image_set.~StyleImageSet_Body(); break;
      default: break;
    }
  }

  StyleGenericImage(const StyleGenericImage& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Url: ::new (&url) (StyleUrl_Body)(other.url); break;
      case Tag::Gradient: ::new (&gradient) (StyleGradient_Body)(other.gradient); break;
      case Tag::Rect: ::new (&rect) (StyleRect_Body)(other.rect); break;
#if defined(CBINDGEN_IS_GECKO)
      case Tag::Element: ::new (&element) (StyleElement_Body)(other.element); break;
#endif
#if defined(CBINDGEN_IS_SERVO)
      case Tag::PaintWorklet: ::new (&paint_worklet) (StylePaintWorklet_Body)(other.paint_worklet); break;
#endif
      case Tag::CrossFade: ::new (&cross_fade) (StyleCrossFade_Body)(other.cross_fade); break;
      case Tag::ImageSet: ::new (&image_set) (StyleImageSet_Body)(other.image_set); break;
      default: break;
    }
  }
  StyleGenericImage& operator=(const StyleGenericImage& other) {
    if (this != &other) {
      this->~StyleGenericImage();
      new (this) StyleGenericImage(other);
    }
    return *this;
  }
 public:
  // If this is an image-set(), the final image we've selected, otherwise it
  // returns *this.
  const StyleGenericImage& FinalImage() const;

  // Whether this image may have an image request associated with it.
  bool IsImageRequestType() const;

  // Gets the image request URL.
  const StyleComputedImageUrl* GetImageRequestURLValue() const;

  // Gets the image data of this image if it has any image request.
  imgRequestProxy* GetImageRequest() const;

  // Returns true if this image is fully loaded, and its size is calculated.
  // Always returns true if there's no image request involved and this image
  // is not `none`.
  bool IsComplete() const;

  // Returns true if this image has an available size and hasn't errored.
  // Always returns true if there's no image request involved and this image
  // is not `none`.
  bool IsSizeAvailable() const;

  // Returns true if the item is definitely opaque --- i.e., paints every
  // pixel within its bounds opaquely, and the bounds contains at least a pixel.
  bool IsOpaque() const;

  struct ActualCropRect {
    nsIntRect mRect; // in image pixels
    bool mIsEntireImage;
  };

  Maybe<ActualCropRect> ComputeActualCropRect() const;

  // Resolves the underlying image request if any.
  void ResolveImage(dom::Document&, const StyleGenericImage* aOld);

  // Returns whether this image has been resolved.
  bool IsResolved() const;
};

/// Computed values for an image according to CSS-IMAGES.
/// <https://drafts.csswg.org/css-images/#image-values>
using StyleImage = StyleGenericImage<StyleGradient, StyleMozImageRect, StyleComputedImageUrl, StyleColor, StylePercentage, StyleResolution>;

/// The path function defined in css-shape-2.
///
/// https://drafts.csswg.org/css-shapes-2/#funcdef-path
struct StylePath {
  /// The filling rule for the svg path.
  StyleFillRule fill;
  /// The svg path data.
  StyleSVGPathData path;

  bool operator==(const StylePath& other) const {
    return fill == other.fill &&
           path == other.path;
  }
  bool operator!=(const StylePath& other) const {
    return fill != other.fill ||
           path != other.path;
  }
};

/// <https://drafts.fxtf.org/css-masking-1/#typedef-geometry-box>
union StyleShapeGeometryBox {
  enum class Tag : uint8_t {
    /// Depending on which kind of element this style value applied on, the
    /// default value of the reference-box can be different.  For an HTML
    /// element, the default value of reference-box is border-box; for an SVG
    /// element, the default value is fill-box.  Since we can not determine the
    /// default value at parsing time, we keep this value to make a decision on
    /// it.
    ElementDependent,
    FillBox,
    StrokeBox,
    ViewBox,
    ShapeBox,
  };

  struct ShapeBox_Body {
    Tag tag;
    StyleShapeBox _0;

    bool operator==(const ShapeBox_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const ShapeBox_Body& other) const {
      return _0 != other._0;
    }
  };

  struct {
    Tag tag;
  };
  ShapeBox_Body shape_box;

  static StyleShapeGeometryBox ElementDependent() {
    StyleShapeGeometryBox result;
    result.tag = Tag::ElementDependent;
    return result;
  }

  bool IsElementDependent() const {
    return tag == Tag::ElementDependent;
  }

  static StyleShapeGeometryBox FillBox() {
    StyleShapeGeometryBox result;
    result.tag = Tag::FillBox;
    return result;
  }

  bool IsFillBox() const {
    return tag == Tag::FillBox;
  }

  static StyleShapeGeometryBox StrokeBox() {
    StyleShapeGeometryBox result;
    result.tag = Tag::StrokeBox;
    return result;
  }

  bool IsStrokeBox() const {
    return tag == Tag::StrokeBox;
  }

  static StyleShapeGeometryBox ViewBox() {
    StyleShapeGeometryBox result;
    result.tag = Tag::ViewBox;
    return result;
  }

  bool IsViewBox() const {
    return tag == Tag::ViewBox;
  }

  static StyleShapeGeometryBox ShapeBox(const StyleShapeBox &_0) {
    StyleShapeGeometryBox result;
    ::new (&result.shape_box._0) (StyleShapeBox)(_0);
    result.tag = Tag::ShapeBox;
    return result;
  }

  bool IsShapeBox() const {
    return tag == Tag::ShapeBox;
  }

  const StyleShapeBox& AsShapeBox() const {
    MOZ_DIAGNOSTIC_ASSERT(IsShapeBox());
    return shape_box._0;
  }

  bool operator==(const StyleShapeGeometryBox& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::ShapeBox: return shape_box == other.shape_box;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleShapeGeometryBox& other) const {
    return !(*this == other);
  }

  private:
  StyleShapeGeometryBox() {

  }
  public:


  ~StyleShapeGeometryBox() {
    switch (tag) {
      case Tag::ShapeBox: shape_box.~ShapeBox_Body(); break;
      default: break;
    }
  }

  StyleShapeGeometryBox(const StyleShapeGeometryBox& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::ShapeBox: ::new (&shape_box) (ShapeBox_Body)(other.shape_box); break;
      default: break;
    }
  }
  StyleShapeGeometryBox& operator=(const StyleShapeGeometryBox& other) {
    if (this != &other) {
      this->~StyleShapeGeometryBox();
      new (this) StyleShapeGeometryBox(other);
    }
    return *this;
  }
};

/// A value for the `clip-path` property.
template<typename BasicShape, typename U>
union StyleGenericClipPath {
  enum class Tag : uint8_t {
    None,
    Url,
    Path,
    Shape,
    Box,
  };

  struct Url_Body {
    Tag tag;
    U _0;

    bool operator==(const Url_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Url_Body& other) const {
      return _0 != other._0;
    }
  };

  struct Path_Body {
    Tag tag;
    StylePath _0;

    bool operator==(const Path_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Path_Body& other) const {
      return _0 != other._0;
    }
  };

  struct Shape_Body {
    Tag tag;
    StyleBox<BasicShape> _0;
    StyleShapeGeometryBox _1;

    bool operator==(const Shape_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const Shape_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct Box_Body {
    Tag tag;
    StyleShapeGeometryBox _0;

    bool operator==(const Box_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Box_Body& other) const {
      return _0 != other._0;
    }
  };

  struct {
    Tag tag;
  };
  Url_Body url;
  Path_Body path;
  Shape_Body shape;
  Box_Body box;

  static StyleGenericClipPath None() {
    StyleGenericClipPath result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericClipPath Url(const U &_0) {
    StyleGenericClipPath result;
    ::new (&result.url._0) (U)(_0);
    result.tag = Tag::Url;
    return result;
  }

  bool IsUrl() const {
    return tag == Tag::Url;
  }

  const U& AsUrl() const {
    MOZ_DIAGNOSTIC_ASSERT(IsUrl());
    return url._0;
  }

  static StyleGenericClipPath Path(const StylePath &_0) {
    StyleGenericClipPath result;
    ::new (&result.path._0) (StylePath)(_0);
    result.tag = Tag::Path;
    return result;
  }

  bool IsPath() const {
    return tag == Tag::Path;
  }

  const StylePath& AsPath() const {
    MOZ_DIAGNOSTIC_ASSERT(IsPath());
    return path._0;
  }

  static StyleGenericClipPath Shape(const StyleBox<BasicShape> &_0,
                                    const StyleShapeGeometryBox &_1) {
    StyleGenericClipPath result;
    ::new (&result.shape._0) (StyleBox<BasicShape>)(_0);
    ::new (&result.shape._1) (StyleShapeGeometryBox)(_1);
    result.tag = Tag::Shape;
    return result;
  }

  bool IsShape() const {
    return tag == Tag::Shape;
  }

  const Shape_Body& AsShape() const {
    MOZ_DIAGNOSTIC_ASSERT(IsShape());
    return shape;
  }

  static StyleGenericClipPath Box(const StyleShapeGeometryBox &_0) {
    StyleGenericClipPath result;
    ::new (&result.box._0) (StyleShapeGeometryBox)(_0);
    result.tag = Tag::Box;
    return result;
  }

  bool IsBox() const {
    return tag == Tag::Box;
  }

  const StyleShapeGeometryBox& AsBox() const {
    MOZ_DIAGNOSTIC_ASSERT(IsBox());
    return box._0;
  }

  bool operator==(const StyleGenericClipPath& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Url: return url == other.url;
      case Tag::Path: return path == other.path;
      case Tag::Shape: return shape == other.shape;
      case Tag::Box: return box == other.box;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericClipPath& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericClipPath() {

  }
  public:


  ~StyleGenericClipPath() {
    switch (tag) {
      case Tag::Url: url.~Url_Body(); break;
      case Tag::Path: path.~Path_Body(); break;
      case Tag::Shape: shape.~Shape_Body(); break;
      case Tag::Box: box.~Box_Body(); break;
      default: break;
    }
  }

  StyleGenericClipPath(const StyleGenericClipPath& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Url: ::new (&url) (Url_Body)(other.url); break;
      case Tag::Path: ::new (&path) (Path_Body)(other.path); break;
      case Tag::Shape: ::new (&shape) (Shape_Body)(other.shape); break;
      case Tag::Box: ::new (&box) (Box_Body)(other.box); break;
      default: break;
    }
  }
  StyleGenericClipPath& operator=(const StyleGenericClipPath& other) {
    if (this != &other) {
      this->~StyleGenericClipPath();
      new (this) StyleGenericClipPath(other);
    }
    return *this;
  }
};

/// A computed `clip-path` value.
using StyleClipPath = StyleGenericClipPath<StyleBasicShape, StyleComputedUrl>;

/// A value for the `shape-outside` property.
template<typename BasicShape, typename I>
union StyleGenericShapeOutside {
  enum class Tag : uint8_t {
    None,
    Image,
    Shape,
    Box,
  };

  struct Image_Body {
    Tag tag;
    I _0;

    bool operator==(const Image_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Image_Body& other) const {
      return _0 != other._0;
    }
  };

  struct Shape_Body {
    Tag tag;
    StyleBox<BasicShape> _0;
    StyleShapeBox _1;

    bool operator==(const Shape_Body& other) const {
      return _0 == other._0 &&
             _1 == other._1;
    }
    bool operator!=(const Shape_Body& other) const {
      return _0 != other._0 ||
             _1 != other._1;
    }
  };

  struct Box_Body {
    Tag tag;
    StyleShapeBox _0;

    bool operator==(const Box_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const Box_Body& other) const {
      return _0 != other._0;
    }
  };

  struct {
    Tag tag;
  };
  Image_Body image;
  Shape_Body shape;
  Box_Body box;

  static StyleGenericShapeOutside None() {
    StyleGenericShapeOutside result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StyleGenericShapeOutside Image(const I &_0) {
    StyleGenericShapeOutside result;
    ::new (&result.image._0) (I)(_0);
    result.tag = Tag::Image;
    return result;
  }

  bool IsImage() const {
    return tag == Tag::Image;
  }

  const I& AsImage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsImage());
    return image._0;
  }

  static StyleGenericShapeOutside Shape(const StyleBox<BasicShape> &_0,
                                        const StyleShapeBox &_1) {
    StyleGenericShapeOutside result;
    ::new (&result.shape._0) (StyleBox<BasicShape>)(_0);
    ::new (&result.shape._1) (StyleShapeBox)(_1);
    result.tag = Tag::Shape;
    return result;
  }

  bool IsShape() const {
    return tag == Tag::Shape;
  }

  const Shape_Body& AsShape() const {
    MOZ_DIAGNOSTIC_ASSERT(IsShape());
    return shape;
  }

  static StyleGenericShapeOutside Box(const StyleShapeBox &_0) {
    StyleGenericShapeOutside result;
    ::new (&result.box._0) (StyleShapeBox)(_0);
    result.tag = Tag::Box;
    return result;
  }

  bool IsBox() const {
    return tag == Tag::Box;
  }

  const StyleShapeBox& AsBox() const {
    MOZ_DIAGNOSTIC_ASSERT(IsBox());
    return box._0;
  }

  bool operator==(const StyleGenericShapeOutside& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Image: return image == other.image;
      case Tag::Shape: return shape == other.shape;
      case Tag::Box: return box == other.box;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericShapeOutside& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericShapeOutside() {

  }
  public:


  ~StyleGenericShapeOutside() {
    switch (tag) {
      case Tag::Image: image.~Image_Body(); break;
      case Tag::Shape: shape.~Shape_Body(); break;
      case Tag::Box: box.~Box_Body(); break;
      default: break;
    }
  }

  StyleGenericShapeOutside(const StyleGenericShapeOutside& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Image: ::new (&image) (Image_Body)(other.image); break;
      case Tag::Shape: ::new (&shape) (Shape_Body)(other.shape); break;
      case Tag::Box: ::new (&box) (Box_Body)(other.box); break;
      default: break;
    }
  }
  StyleGenericShapeOutside& operator=(const StyleGenericShapeOutside& other) {
    if (this != &other) {
      this->~StyleGenericShapeOutside();
      new (this) StyleGenericShapeOutside(other);
    }
    return *this;
  }
};

/// A computed `shape-outside` value.
using StyleShapeOutside = StyleGenericShapeOutside<StyleBasicShape, StyleImage>;

/// Controls how the auto-placement algorithm works
/// specifying exactly how auto-placed items get flowed into the grid
struct StyleGridAutoFlow {
  uint8_t bits;

  explicit operator bool() const {
    return !!bits;
  }
  StyleGridAutoFlow operator~() const {
    return {static_cast<decltype(bits)>(~bits)};
  }
  StyleGridAutoFlow operator|(const StyleGridAutoFlow& other) const {
    return {static_cast<decltype(bits)>(this->bits | other.bits)};
  }
  StyleGridAutoFlow& operator|=(const StyleGridAutoFlow& other) {
    *this = (*this | other);
    return *this;
  }
  StyleGridAutoFlow operator&(const StyleGridAutoFlow& other) const {
    return {static_cast<decltype(bits)>(this->bits & other.bits)};
  }
  StyleGridAutoFlow& operator&=(const StyleGridAutoFlow& other) {
    *this = (*this & other);
    return *this;
  }
  StyleGridAutoFlow operator^(const StyleGridAutoFlow& other) const {
    return {static_cast<decltype(bits)>(this->bits ^ other.bits)};
  }
  StyleGridAutoFlow& operator^=(const StyleGridAutoFlow& other) {
    *this = (*this ^ other);
    return *this;
  }
  bool operator==(const StyleGridAutoFlow& other) const {
    return bits == other.bits;
  }
  bool operator!=(const StyleGridAutoFlow& other) const {
    return bits != other.bits;
  }
  static const StyleGridAutoFlow ROW;
  static const StyleGridAutoFlow COLUMN;
  static const StyleGridAutoFlow DENSE;
};
/// 'row' - mutually exclusive with 'column'
inline const StyleGridAutoFlow StyleGridAutoFlow::ROW = StyleGridAutoFlow{ /* .bits = */ (uint8_t)(1 << 0) };
/// 'column' - mutually exclusive with 'row'
inline const StyleGridAutoFlow StyleGridAutoFlow::COLUMN = StyleGridAutoFlow{ /* .bits = */ (uint8_t)(1 << 1) };
/// 'dense'
inline const StyleGridAutoFlow StyleGridAutoFlow::DENSE = StyleGridAutoFlow{ /* .bits = */ (uint8_t)(1 << 2) };

/// A generic value for item of `image cursors`.
template<typename ImageUrl, typename Number>
struct StyleGenericCursorImage {
  /// The url to parse images from.
  ImageUrl url;
  /// Whether the image has a hotspot or not.
  bool has_hotspot;
  /// The x coordinate.
  Number hotspot_x;
  /// The y coordinate.
  Number hotspot_y;

  bool operator==(const StyleGenericCursorImage& other) const {
    return url == other.url &&
           has_hotspot == other.has_hotspot &&
           hotspot_x == other.hotspot_x &&
           hotspot_y == other.hotspot_y;
  }
  bool operator!=(const StyleGenericCursorImage& other) const {
    return url != other.url ||
           has_hotspot != other.has_hotspot ||
           hotspot_x != other.hotspot_x ||
           hotspot_y != other.hotspot_y;
  }
};

/// A computed value for item of `image cursors`.
using StyleCursorImage = StyleGenericCursorImage<StyleComputedImageUrl, StyleNumber>;

/// A generic value for the `cursor` property.
///
/// https://drafts.csswg.org/css-ui/#cursor
template<typename Image>
struct StyleGenericCursor {
  /// The parsed images for the cursor.
  StyleOwnedSlice<Image> images;
  /// The kind of the cursor [default | help | ...].
  StyleCursorKind keyword;

  bool operator==(const StyleGenericCursor& other) const {
    return images == other.images &&
           keyword == other.keyword;
  }
  bool operator!=(const StyleGenericCursor& other) const {
    return images != other.images ||
           keyword != other.keyword;
  }
};

/// A computed value for the `cursor` property.
using StyleCursor = StyleGenericCursor<StyleCursorImage>;

/// Generic value for stroke-dasharray.
template<typename L>
struct StyleGenericSVGStrokeDashArray {
  enum class Tag : uint8_t {
    /// `[ <length> | <percentage> | <number> ]#`
    Values,
    /// `context-value`
    ContextValue,
  };

  struct StyleValues_Body {
    StyleOwnedSlice<L> _0;

    bool operator==(const StyleValues_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleValues_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleValues_Body values;
  };

  static StyleGenericSVGStrokeDashArray Values(const StyleOwnedSlice<L> &_0) {
    StyleGenericSVGStrokeDashArray result;
    ::new (&result.values._0) (StyleOwnedSlice<L>)(_0);
    result.tag = Tag::Values;
    return result;
  }

  bool IsValues() const {
    return tag == Tag::Values;
  }

  const StyleOwnedSlice<L>& AsValues() const {
    MOZ_DIAGNOSTIC_ASSERT(IsValues());
    return values._0;
  }

  static StyleGenericSVGStrokeDashArray ContextValue() {
    StyleGenericSVGStrokeDashArray result;
    result.tag = Tag::ContextValue;
    return result;
  }

  bool IsContextValue() const {
    return tag == Tag::ContextValue;
  }

  bool operator==(const StyleGenericSVGStrokeDashArray& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Values: return values == other.values;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericSVGStrokeDashArray& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericSVGStrokeDashArray() {

  }
  public:


  ~StyleGenericSVGStrokeDashArray() {
    switch (tag) {
      case Tag::Values: values.~StyleValues_Body(); break;
      default: break;
    }
  }

  StyleGenericSVGStrokeDashArray(const StyleGenericSVGStrokeDashArray& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Values: ::new (&values) (StyleValues_Body)(other.values); break;
      default: break;
    }
  }
  StyleGenericSVGStrokeDashArray& operator=(const StyleGenericSVGStrokeDashArray& other) {
    if (this != &other) {
      this->~StyleGenericSVGStrokeDashArray();
      new (this) StyleGenericSVGStrokeDashArray(other);
    }
    return *this;
  }
};

/// [ <length> | <percentage> | <number> ]# | context-value
using StyleSVGStrokeDashArray = StyleGenericSVGStrokeDashArray<StyleNonNegativeLengthPercentage>;

/// An SVG length value supports `context-value` in addition to length.
template<typename L>
struct StyleGenericSVGLength {
  enum class Tag : uint8_t {
    /// `<length> | <percentage> | <number>`
    LengthPercentage,
    /// `context-value`
    ContextValue,
  };

  struct StyleLengthPercentage_Body {
    L _0;

    bool operator==(const StyleLengthPercentage_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleLengthPercentage_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleLengthPercentage_Body length_percentage;
  };

  static StyleGenericSVGLength LengthPercentage(const L &_0) {
    StyleGenericSVGLength result;
    ::new (&result.length_percentage._0) (L)(_0);
    result.tag = Tag::LengthPercentage;
    return result;
  }

  bool IsLengthPercentage() const {
    return tag == Tag::LengthPercentage;
  }

  const L& AsLengthPercentage() const {
    MOZ_DIAGNOSTIC_ASSERT(IsLengthPercentage());
    return length_percentage._0;
  }

  static StyleGenericSVGLength ContextValue() {
    StyleGenericSVGLength result;
    result.tag = Tag::ContextValue;
    return result;
  }

  bool IsContextValue() const {
    return tag == Tag::ContextValue;
  }

  bool operator==(const StyleGenericSVGLength& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::LengthPercentage: return length_percentage == other.length_percentage;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericSVGLength& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericSVGLength() {

  }
  public:


  ~StyleGenericSVGLength() {
    switch (tag) {
      case Tag::LengthPercentage: length_percentage.~StyleLengthPercentage_Body(); break;
      default: break;
    }
  }

  StyleGenericSVGLength(const StyleGenericSVGLength& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::LengthPercentage: ::new (&length_percentage) (StyleLengthPercentage_Body)(other.length_percentage); break;
      default: break;
    }
  }
  StyleGenericSVGLength& operator=(const StyleGenericSVGLength& other) {
    if (this != &other) {
      this->~StyleGenericSVGLength();
      new (this) StyleGenericSVGLength(other);
    }
    return *this;
  }
};

/// <length> | <percentage> | <number> | context-value
using StyleSVGLength = StyleGenericSVGLength<StyleLengthPercentage>;

/// A type used for opacity.
using StyleOpacity = StyleCSSFloat;

/// An SVG opacity value accepts `context-{fill,stroke}-opacity` in
/// addition to opacity value.
template<typename OpacityType>
struct StyleGenericSVGOpacity {
  enum class Tag : uint8_t {
    /// `<opacity-value>`
    Opacity,
    /// `context-fill-opacity`
    ContextFillOpacity,
    /// `context-stroke-opacity`
    ContextStrokeOpacity,
  };

  struct StyleOpacity_Body {
    OpacityType _0;

    bool operator==(const StyleOpacity_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleOpacity_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleOpacity_Body opacity;
  };

  static StyleGenericSVGOpacity Opacity(const OpacityType &_0) {
    StyleGenericSVGOpacity result;
    ::new (&result.opacity._0) (OpacityType)(_0);
    result.tag = Tag::Opacity;
    return result;
  }

  bool IsOpacity() const {
    return tag == Tag::Opacity;
  }

  const OpacityType& AsOpacity() const {
    MOZ_DIAGNOSTIC_ASSERT(IsOpacity());
    return opacity._0;
  }

  static StyleGenericSVGOpacity ContextFillOpacity() {
    StyleGenericSVGOpacity result;
    result.tag = Tag::ContextFillOpacity;
    return result;
  }

  bool IsContextFillOpacity() const {
    return tag == Tag::ContextFillOpacity;
  }

  static StyleGenericSVGOpacity ContextStrokeOpacity() {
    StyleGenericSVGOpacity result;
    result.tag = Tag::ContextStrokeOpacity;
    return result;
  }

  bool IsContextStrokeOpacity() const {
    return tag == Tag::ContextStrokeOpacity;
  }

  bool operator==(const StyleGenericSVGOpacity& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Opacity: return opacity == other.opacity;
      default: break;
    }
    return true;
  }

  bool operator!=(const StyleGenericSVGOpacity& other) const {
    return !(*this == other);
  }

  private:
  StyleGenericSVGOpacity() {

  }
  public:


  ~StyleGenericSVGOpacity() {
    switch (tag) {
      case Tag::Opacity: opacity.~StyleOpacity_Body(); break;
      default: break;
    }
  }

  StyleGenericSVGOpacity(const StyleGenericSVGOpacity& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Opacity: ::new (&opacity) (StyleOpacity_Body)(other.opacity); break;
      default: break;
    }
  }
  StyleGenericSVGOpacity& operator=(const StyleGenericSVGOpacity& other) {
    if (this != &other) {
      this->~StyleGenericSVGOpacity();
      new (this) StyleGenericSVGOpacity(other);
    }
    return *this;
  }
};

/// <opacity-value> | context-fill-opacity | context-stroke-opacity
using StyleSVGOpacity = StyleGenericSVGOpacity<StyleOpacity>;

/// An non-negative wrapper of SVGLength.
using StyleSVGWidth = StyleGenericSVGLength<StyleNonNegativeLengthPercentage>;

/// The computed value of `text-align`.
using StyleTextAlign = StyleTextAlignKeyword;

/// A generic value for the `<ratio>` value.
template<typename N>
struct StyleRatio {
  N _0;
  N _1;

  bool operator==(const StyleRatio& other) const {
    return _0 == other._0 &&
           _1 == other._1;
  }
  bool operator!=(const StyleRatio& other) const {
    return _0 != other._0 ||
           _1 != other._1;
  }
  inline AspectRatio ToLayoutRatio(UseBoxSizing aUseBoxSizing) const;
};

/// Ratio or None.
template<typename N>
struct StylePreferredRatio {
  enum class Tag : uint8_t {
    /// Without specified ratio
    None,
    /// With specified ratio
    Ratio,
  };

  struct StyleRatio_Body {
    StyleRatio<N> _0;

    bool operator==(const StyleRatio_Body& other) const {
      return _0 == other._0;
    }
    bool operator!=(const StyleRatio_Body& other) const {
      return _0 != other._0;
    }
  };

  Tag tag;
  union {
    StyleRatio_Body ratio;
  };

  static StylePreferredRatio None() {
    StylePreferredRatio result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }

  static StylePreferredRatio Ratio(const StyleRatio<N> &_0) {
    StylePreferredRatio result;
    ::new (&result.ratio._0) (StyleRatio<N>)(_0);
    result.tag = Tag::Ratio;
    return result;
  }

  bool IsRatio() const {
    return tag == Tag::Ratio;
  }

  const StyleRatio<N>& AsRatio() const {
    MOZ_DIAGNOSTIC_ASSERT(IsRatio());
    return ratio._0;
  }

  bool operator==(const StylePreferredRatio& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Ratio: return ratio == other.ratio;
      default: break;
    }
    return true;
  }

  bool operator!=(const StylePreferredRatio& other) const {
    return !(*this == other);
  }

  private:
  StylePreferredRatio() {

  }
  public:


  ~StylePreferredRatio() {
    switch (tag) {
      case Tag::Ratio: ratio.~StyleRatio_Body(); break;
      default: break;
    }
  }

  StylePreferredRatio(const StylePreferredRatio& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Ratio: ::new (&ratio) (StyleRatio_Body)(other.ratio); break;
      default: break;
    }
  }
  StylePreferredRatio& operator=(const StylePreferredRatio& other) {
    if (this != &other) {
      this->~StylePreferredRatio();
      new (this) StylePreferredRatio(other);
    }
    return *this;
  }
};

/// A generic value for the `aspect-ratio` property, the value is `auto || <ratio>`.
template<typename N>
struct StyleGenericAspectRatio {
  /// Specifiy auto or not.
  bool auto_;
  /// The preferred aspect-ratio value.
  StylePreferredRatio<N> ratio;

  bool operator==(const StyleGenericAspectRatio& other) const {
    return auto_ == other.auto_ &&
           ratio == other.ratio;
  }
  bool operator!=(const StyleGenericAspectRatio& other) const {
    return auto_ != other.auto_ ||
           ratio != other.ratio;
  }
  bool HasRatio() const { return ratio.IsRatio(); }
  bool HasFiniteRatio() const { return static_cast<bool>(ToLayoutRatio()); }
  bool BehavesAsAuto() const { return auto_ || !HasFiniteRatio(); }
  inline AspectRatio ToLayoutRatio() const;

  static StyleGenericAspectRatio Auto() {
    return {true, StylePreferredRatio<N>::None()};
  }
};

/// A computed value for the `aspect-ratio` property.
using StyleAspectRatio = StyleGenericAspectRatio<StyleNonNegativeNumber>;

#if defined(CBINDGEN_IS_GECKO)
/// The default font sizes for generic families for a given language group.
struct StyleDefaultFontSizes {
  StyleLength variable;
  StyleLength serif;
  StyleLength sans_serif;
  StyleLength monospace;
  StyleLength cursive;
  StyleLength fantasy;

  bool operator==(const StyleDefaultFontSizes& other) const {
    return variable == other.variable &&
           serif == other.serif &&
           sans_serif == other.sans_serif &&
           monospace == other.monospace &&
           cursive == other.cursive &&
           fantasy == other.fantasy;
  }
  bool operator!=(const StyleDefaultFontSizes& other) const {
    return variable != other.variable ||
           serif != other.serif ||
           sans_serif != other.sans_serif ||
           monospace != other.monospace ||
           cursive != other.cursive ||
           fantasy != other.fantasy;
  }
};
#endif





















































































































































/// Number of app units per pixel
static const StyleCSSFloat StyleAU_PER_PX = 60.;

/// Number of app units per inch
static const StyleCSSFloat StyleAU_PER_IN = (StyleAU_PER_PX * 96.);

/// Number of app units per centimeter
static const StyleCSSFloat StyleAU_PER_CM = (StyleAU_PER_IN / 2.54);

/// Number of app units per millimeter
static const StyleCSSFloat StyleAU_PER_MM = (StyleAU_PER_IN / 25.4);

/// Number of app units per quarter
static const StyleCSSFloat StyleAU_PER_Q = (StyleAU_PER_MM / 4.);

/// Number of app units per point
static const StyleCSSFloat StyleAU_PER_PT = (StyleAU_PER_IN / 72.);

/// Number of app units per pica
static const StyleCSSFloat StyleAU_PER_PC = (StyleAU_PER_PT * 12.);



























extern "C" {

size_t je_malloc_usable_size(const void*);

void Servo_Initialize(URLExtraData *dummy_url_data,
                      URLExtraData *dummy_chrome_url_data);

void Servo_Shutdown();

/// Traverses the subtree rooted at `root` for restyling.
///
/// Returns whether the root was restyled. Whether anything else was restyled or
/// not can be inferred from the dirty bits in the rest of the tree.
bool Servo_TraverseSubtree(const StyleRawGeckoElement *root,
                           const StyleRawServoStyleSet *raw_data,
                           const ServoElementSnapshotTable *snapshots,
                           ServoTraversalFlags raw_flags);

/// Checks whether the rule tree has crossed its threshold for unused nodes, and
/// if so, frees them.
void Servo_MaybeGCRuleTree(const StyleRawServoStyleSet *raw_data);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValues_Interpolate(const StyleRawServoAnimationValue *from,
                                                                           const StyleRawServoAnimationValue *to,
                                                                           double progress);

bool Servo_AnimationValues_IsInterpolable(const StyleRawServoAnimationValue *from,
                                          const StyleRawServoAnimationValue *to);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValues_Add(const StyleRawServoAnimationValue *a,
                                                                   const StyleRawServoAnimationValue *b);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValues_Accumulate(const StyleRawServoAnimationValue *a,
                                                                          const StyleRawServoAnimationValue *b,
                                                                          uint64_t count);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValues_GetZeroValue(const StyleRawServoAnimationValue *value_to_match);

double Servo_AnimationValues_ComputeDistance(const StyleRawServoAnimationValue *from,
                                             const StyleRawServoAnimationValue *to);

StyleStrong<StyleRawServoAnimationValue> Servo_ComposeAnimationSegment(const AnimationPropertySegment *segment,
                                                                       const StyleRawServoAnimationValue *underlying_value,
                                                                       const StyleRawServoAnimationValue *last_value,
                                                                       StyleIterationCompositeOperation iteration_composite,
                                                                       double progress,
                                                                       uint64_t current_iteration);

void Servo_AnimationCompose(RawServoAnimationValueMap *raw_value_map,
                            const RawServoAnimationValueTable *base_values,
                            nsCSSPropertyID css_property,
                            const AnimationPropertySegment *segment,
                            const AnimationPropertySegment *last_segment,
                            const ComputedTiming *computed_timing,
                            StyleIterationCompositeOperation iteration_composite);

void Servo_AnimationValue_Serialize(const StyleRawServoAnimationValue *value,
                                    nsCSSPropertyID property,
                                    const StyleRawServoStyleSet *raw_data,
                                    nsACString *buffer);

/// Debug: MOZ_DBG for AnimationValue.
void Servo_AnimationValue_Dump(const StyleRawServoAnimationValue *value,
                               nsACString *result);

nscolor Servo_AnimationValue_GetColor(const StyleRawServoAnimationValue *value,
                                      nscolor foreground_color);

bool Servo_AnimationValue_IsCurrentColor(const StyleRawServoAnimationValue *value);

float Servo_AnimationValue_GetOpacity(const StyleRawServoAnimationValue *value);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_Opacity(float opacity);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_Color(nsCSSPropertyID color_property,
                                                                    nscolor color);

const StyleScale *Servo_AnimationValue_GetScale(const StyleRawServoAnimationValue *value);

const StyleTranslate *Servo_AnimationValue_GetTranslate(const StyleRawServoAnimationValue *value);

const StyleRotate *Servo_AnimationValue_GetRotate(const StyleRawServoAnimationValue *value);

const StyleTransform *Servo_AnimationValue_GetTransform(const StyleRawServoAnimationValue *value);

const StyleOffsetPath *Servo_AnimationValue_GetOffsetPath(const StyleRawServoAnimationValue *value);

const StyleLengthPercentage *Servo_AnimationValue_GetOffsetDistance(const StyleRawServoAnimationValue *value);

const StyleOffsetRotate *Servo_AnimationValue_GetOffsetRotate(const StyleRawServoAnimationValue *value);

const StylePositionOrAuto *Servo_AnimationValue_GetOffsetAnchor(const StyleRawServoAnimationValue *value);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_Rotate(const StyleRotate *r);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_Translate(const StyleTranslate *t);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_Scale(const StyleScale *s);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_Transform(const StyleTransform *transform);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_OffsetPath(const StyleOffsetPath *p);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_OffsetDistance(const StyleLengthPercentage *d);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_OffsetRotate(const StyleOffsetRotate *r);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_OffsetAnchor(const StylePositionOrAuto *p);

bool Servo_AnimationValue_DeepEqual(const StyleRawServoAnimationValue *this_,
                                    const StyleRawServoAnimationValue *other);

StyleStrong<StyleRawServoDeclarationBlock> Servo_AnimationValue_Uncompute(const StyleRawServoAnimationValue *value);

void Servo_SVGPathData_Normalize(const StyleSVGPathData *input,
                                 StyleSVGPathData *output);

StyleOwned<RawServoAnimationValueMap> Servo_AnimationValueMap_Create();

void Servo_AnimationValueMap_Drop(RawServoAnimationValueMap *value_map);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValueMap_GetValue(RawServoAnimationValueMap *raw_value_map,
                                                                          nsCSSPropertyID property_id);

StyleStrong<StyleComputedValues> Servo_StyleSet_GetBaseComputedValuesForElement(const StyleRawServoStyleSet *raw_style_set,
                                                                                const StyleRawGeckoElement *element,
                                                                                const StyleComputedValues *computed_values,
                                                                                const ServoElementSnapshotTable *snapshots);

StyleStrong<StyleComputedValues> Servo_StyleSet_GetComputedValuesByAddingAnimation(const StyleRawServoStyleSet *raw_style_set,
                                                                                   const StyleRawGeckoElement *element,
                                                                                   const StyleComputedValues *computed_values,
                                                                                   const ServoElementSnapshotTable *snapshots,
                                                                                   const StyleRawServoAnimationValue *animation_value);

StyleStrong<StyleRawServoAnimationValue> Servo_ComputedValues_ExtractAnimationValue(const StyleComputedValues *computed_values,
                                                                                    nsCSSPropertyID property_id);

nsCSSPropertyID Servo_ResolveLogicalProperty(nsCSSPropertyID property_id,
                                             const StyleComputedValues *style);

nsCSSPropertyID Servo_Property_LookupEnabledForAllContent(const nsACString *prop);

const uint8_t *Servo_Property_GetName(nsCSSPropertyID prop,
                                      uint32_t *out_length);

bool Servo_Property_IsShorthand(const nsACString *prop_name, bool *found);

bool Servo_Property_IsInherited(const nsACString *prop_name);

bool Servo_Property_SupportsType(const nsACString *prop_name,
                                 uint8_t ty,
                                 bool *found);

void Servo_Property_GetCSSValuesForProperty(const nsACString *prop_name,
                                            bool *found,
                                            nsTArray<nsString> *result);

bool Servo_Property_IsAnimatable(nsCSSPropertyID prop);

bool Servo_Property_IsTransitionable(nsCSSPropertyID prop);

bool Servo_Property_IsDiscreteAnimatable(nsCSSPropertyID property);

void Servo_Element_ClearData(const StyleRawGeckoElement *element);

uintptr_t Servo_Element_SizeOfExcludingThisAndCVs(StyleGeckoMallocSizeOf malloc_size_of,
                                                  StyleGeckoMallocSizeOf malloc_enclosing_size_of,
                                                  SeenPtrs *seen_ptrs,
                                                  const StyleRawGeckoElement *element);

const StyleComputedValues *Servo_Element_GetMaybeOutOfDateStyle(const StyleRawGeckoElement *element);

const StyleComputedValues *Servo_Element_GetMaybeOutOfDatePseudoStyle(const StyleRawGeckoElement *element,
                                                                      uintptr_t index);

bool Servo_Element_IsDisplayNone(const StyleRawGeckoElement *element);

bool Servo_Element_IsDisplayContents(const StyleRawGeckoElement *element);

bool Servo_Element_IsPrimaryStyleReusedViaRuleNode(const StyleRawGeckoElement *element);

StyleStrong<StyleRawServoStyleSheetContents> Servo_StyleSheet_Empty(StyleSheetParsingMode mode);

/// Note: The load_data corresponds to this sheet, and is passed as the parent
/// load data for child sheet loads. It may be null for certain cases where we
/// know we won't have child loads.
StyleStrong<StyleRawServoStyleSheetContents> Servo_StyleSheet_FromUTF8Bytes(StyleLoader *loader,
                                                                            StyleDomStyleSheet *stylesheet,
                                                                            StyleSheetLoadData *load_data,
                                                                            const nsACString *bytes,
                                                                            StyleSheetParsingMode mode,
                                                                            URLExtraData *extra_data,
                                                                            uint32_t line_number_offset,
                                                                            nsCompatibility quirks_mode,
                                                                            StyleLoaderReusableStyleSheets *reusable_sheets,
                                                                            const StyleUseCounters *use_counters,
                                                                            StyleAllowImportRules allow_import_rules,
                                                                            StyleSanitizationKind sanitization_kind,
                                                                            nsAString *sanitized_output);

void Servo_StyleSheet_FromUTF8BytesAsync(StyleSheetLoadDataHolder *load_data,
                                         URLExtraData *extra_data,
                                         const nsACString *bytes,
                                         StyleSheetParsingMode mode,
                                         uint32_t line_number_offset,
                                         nsCompatibility quirks_mode,
                                         bool should_record_use_counters,
                                         StyleAllowImportRules allow_import_rules);

void Servo_ShutdownThreadPool();

StyleStrong<StyleRawServoStyleSheetContents> Servo_StyleSheet_FromSharedData(URLExtraData *extra_data,
                                                                             const StyleServoCssRules *shared_rules);

void Servo_StyleSet_AppendStyleSheet(const StyleRawServoStyleSet *raw_data,
                                     const StyleDomStyleSheet *sheet);

StyleOwned<StyleRawServoAuthorStyles> Servo_AuthorStyles_Create();

void Servo_AuthorStyles_Drop(StyleRawServoAuthorStyles *styles);

void Servo_AuthorStyles_AppendStyleSheet(StyleRawServoAuthorStyles *styles,
                                         const StyleDomStyleSheet *sheet);

void Servo_AuthorStyles_InsertStyleSheetBefore(StyleRawServoAuthorStyles *styles,
                                               const StyleDomStyleSheet *sheet,
                                               const StyleDomStyleSheet *before_sheet);

void Servo_AuthorStyles_RemoveStyleSheet(StyleRawServoAuthorStyles *styles,
                                         const StyleDomStyleSheet *sheet);

void Servo_AuthorStyles_ForceDirty(StyleRawServoAuthorStyles *styles);

bool Servo_AuthorStyles_IsDirty(const StyleRawServoAuthorStyles *styles);

void Servo_AuthorStyles_Flush(StyleRawServoAuthorStyles *styles,
                              const StyleRawServoStyleSet *document_set);

uintptr_t Servo_DeclarationBlock_SizeOfIncludingThis(StyleGeckoMallocSizeOf malloc_size_of,
                                                     StyleGeckoMallocSizeOf malloc_enclosing_size_of,
                                                     const StyleRawServoDeclarationBlock *declarations);

uintptr_t Servo_AuthorStyles_SizeOfIncludingThis(StyleGeckoMallocSizeOf malloc_size_of,
                                                 StyleGeckoMallocSizeOf malloc_enclosing_size_of,
                                                 const StyleRawServoAuthorStyles *styles);

MediumFeaturesChangedResult Servo_StyleSet_MediumFeaturesChanged(const StyleRawServoStyleSet *document_set,
                                                                 nsTArray<StyleRawServoAuthorStyles*> *non_document_styles,
                                                                 bool may_affect_default_style);

void Servo_StyleSet_InsertStyleSheetBefore(const StyleRawServoStyleSet *raw_data,
                                           const StyleDomStyleSheet *sheet,
                                           const StyleDomStyleSheet *before_sheet);

void Servo_StyleSet_RemoveStyleSheet(const StyleRawServoStyleSet *raw_data,
                                     const StyleDomStyleSheet *sheet);

const StyleDomStyleSheet *Servo_StyleSet_GetSheetAt(const StyleRawServoStyleSet *raw_data,
                                                    StyleOrigin origin,
                                                    uintptr_t index);

uintptr_t Servo_StyleSet_GetSheetCount(const StyleRawServoStyleSet *raw_data,
                                       StyleOrigin origin);

void Servo_StyleSet_FlushStyleSheets(const StyleRawServoStyleSet *raw_data,
                                     const StyleRawGeckoElement *doc_element,
                                     const ServoElementSnapshotTable *snapshots);

void Servo_StyleSet_NoteStyleSheetsChanged(const StyleRawServoStyleSet *raw_data,
                                           OriginFlags changed_origins);

void Servo_StyleSet_SetAuthorStyleDisabled(const StyleRawServoStyleSet *raw_data,
                                           bool author_style_disabled);

bool Servo_StyleSheet_HasRules(const StyleRawServoStyleSheetContents *raw_contents);

StyleStrong<StyleServoCssRules> Servo_StyleSheet_GetRules(const StyleRawServoStyleSheetContents *sheet);

StyleStrong<StyleRawServoStyleSheetContents> Servo_StyleSheet_Clone(const StyleRawServoStyleSheetContents *raw_sheet,
                                                                    const StyleDomStyleSheet *reference_sheet);

uintptr_t Servo_StyleSheet_SizeOfIncludingThis(StyleGeckoMallocSizeOf malloc_size_of,
                                               StyleGeckoMallocSizeOf malloc_enclosing_size_of,
                                               const StyleRawServoStyleSheetContents *sheet);

StyleOrigin Servo_StyleSheet_GetOrigin(const StyleRawServoStyleSheetContents *sheet);

void Servo_StyleSheet_GetSourceMapURL(const StyleRawServoStyleSheetContents *sheet,
                                      nsAString *result);

void Servo_StyleSheet_GetSourceURL(const StyleRawServoStyleSheetContents *sheet,
                                   nsAString *result);

void Servo_CssRules_ListTypes(const StyleServoCssRules *rules,
                              nsTArray<uintptr_t> *result);

nsresult Servo_CssRules_InsertRule(const StyleServoCssRules *rules,
                                   const StyleRawServoStyleSheetContents *contents,
                                   const nsACString *rule,
                                   uint32_t index,
                                   bool nested,
                                   StyleLoader *loader,
                                   StyleAllowImportRules allow_import_rules,
                                   StyleDomStyleSheet *gecko_stylesheet,
                                   uint16_t *rule_type);

nsresult Servo_CssRules_DeleteRule(const StyleServoCssRules *rules,
                                   uint32_t index);

StyleStrong<StyleRawServoDeclarationBlock> Servo_StyleRule_GetStyle(const StyleRawServoStyleRule *rule);

void Servo_StyleRule_SetStyle(const StyleRawServoStyleRule *rule,
                              const StyleRawServoDeclarationBlock *declarations);

void Servo_StyleRule_GetSelectorText(const StyleRawServoStyleRule *rule,
                                     nsACString *result);

void Servo_StyleRule_GetSelectorTextAtIndex(const StyleRawServoStyleRule *rule,
                                            uint32_t index,
                                            nsACString *result);

void Servo_StyleRule_GetSelectorCount(const StyleRawServoStyleRule *rule,
                                      uint32_t *count);

void Servo_StyleRule_GetSpecificityAtIndex(const StyleRawServoStyleRule *rule,
                                           uint32_t index,
                                           uint64_t *specificity);

bool Servo_StyleRule_SelectorMatchesElement(const StyleRawServoStyleRule *rule,
                                            const StyleRawGeckoElement *element,
                                            uint32_t index,
                                            PseudoStyleType pseudo_type,
                                            bool relevant_link_visited);

bool Servo_StyleRule_SetSelectorText(const StyleRawServoStyleSheetContents *sheet,
                                     const StyleRawServoStyleRule *rule,
                                     const nsACString *text);

const StyleRawGeckoElement *Servo_SelectorList_Closest(const StyleRawGeckoElement *element,
                                                       const StyleRawServoSelectorList *selectors);

bool Servo_SelectorList_Matches(const StyleRawGeckoElement *element,
                                const StyleRawServoSelectorList *selectors);

const StyleRawGeckoElement *Servo_SelectorList_QueryFirst(const StyleRawGeckoNode *node,
                                                          const StyleRawServoSelectorList *selectors,
                                                          bool may_use_invalidation);

void Servo_SelectorList_QueryAll(const StyleRawGeckoNode *node,
                                 const StyleRawServoSelectorList *selectors,
                                 nsSimpleContentList *content_list,
                                 bool may_use_invalidation);

void Servo_ImportRule_GetHref(const StyleRawServoImportRule *rule,
                              nsAString *result);

const StyleDomStyleSheet *Servo_ImportRule_GetSheet(const StyleRawServoImportRule *rule);

void Servo_ImportRule_SetSheet(const StyleRawServoImportRule *rule,
                               StyleDomStyleSheet *sheet);

void Servo_Keyframe_GetKeyText(const StyleRawServoKeyframe *keyframe,
                               nsACString *result);

bool Servo_Keyframe_SetKeyText(const StyleRawServoKeyframe *keyframe,
                               const nsACString *text);

StyleStrong<StyleRawServoDeclarationBlock> Servo_Keyframe_GetStyle(const StyleRawServoKeyframe *keyframe);

void Servo_Keyframe_SetStyle(const StyleRawServoKeyframe *keyframe,
                             const StyleRawServoDeclarationBlock *declarations);

nsAtom *Servo_KeyframesRule_GetName(const StyleRawServoKeyframesRule *rule);

void Servo_KeyframesRule_SetName(const StyleRawServoKeyframesRule *rule,
                                 nsAtom *name);

uint32_t Servo_KeyframesRule_GetCount(const StyleRawServoKeyframesRule *rule);

StyleStrong<StyleRawServoKeyframe> Servo_KeyframesRule_GetKeyframeAt(const StyleRawServoKeyframesRule *rule,
                                                                     uint32_t index,
                                                                     uint32_t *line,
                                                                     uint32_t *column);

uint32_t Servo_KeyframesRule_FindRule(const StyleRawServoKeyframesRule *rule,
                                      const nsACString *key);

bool Servo_KeyframesRule_AppendRule(const StyleRawServoKeyframesRule *rule,
                                    const StyleRawServoStyleSheetContents *contents,
                                    const nsACString *css);

void Servo_KeyframesRule_DeleteRule(const StyleRawServoKeyframesRule *rule,
                                    uint32_t index);

StyleStrong<StyleRawServoMediaList> Servo_MediaRule_GetMedia(const StyleRawServoMediaRule *rule);

nsAtom *Servo_NamespaceRule_GetPrefix(const StyleRawServoNamespaceRule *rule);

nsAtom *Servo_NamespaceRule_GetURI(const StyleRawServoNamespaceRule *rule);

StyleStrong<StyleRawServoDeclarationBlock> Servo_PageRule_GetStyle(const StyleRawServoPageRule *rule);

void Servo_PageRule_SetStyle(const StyleRawServoPageRule *rule,
                             const StyleRawServoDeclarationBlock *declarations);

void Servo_SupportsRule_GetConditionText(const StyleRawServoSupportsRule *rule,
                                         nsACString *result);

void Servo_MozDocumentRule_GetConditionText(const StyleRawServoMozDocumentRule *rule,
                                            nsACString *result);

void Servo_FontFeatureValuesRule_GetFontFamily(const StyleRawServoFontFeatureValuesRule *rule,
                                               nsACString *result);

void Servo_FontFeatureValuesRule_GetValueText(const StyleRawServoFontFeatureValuesRule *rule,
                                              nsACString *result);

StyleStrong<StyleRawServoFontFaceRule> Servo_FontFaceRule_CreateEmpty();

StyleStrong<StyleRawServoFontFaceRule> Servo_FontFaceRule_Clone(const StyleRawServoFontFaceRule *rule);

void Servo_FontFaceRule_GetSourceLocation(const StyleRawServoFontFaceRule *rule,
                                          uint32_t *line,
                                          uint32_t *column);

uint32_t Servo_FontFaceRule_Length(const StyleRawServoFontFaceRule *rule);

nsCSSFontDesc Servo_FontFaceRule_IndexGetter(const StyleRawServoFontFaceRule *rule,
                                             uint32_t index);

void Servo_FontFaceRule_GetDeclCssText(const StyleRawServoFontFaceRule *rule,
                                       nsACString *result);

bool Servo_FontFaceRule_GetFontWeight(const StyleRawServoFontFaceRule *rule,
                                      StyleComputedFontWeightRange *out);

bool Servo_FontFaceRule_GetFontStretch(const StyleRawServoFontFaceRule *rule,
                                       StyleComputedFontStretchRange *out);

bool Servo_FontFaceRule_GetFontStyle(const StyleRawServoFontFaceRule *rule,
                                     StyleComputedFontStyleDescriptor *out);

bool Servo_FontFaceRule_GetFontDisplay(const StyleRawServoFontFaceRule *rule,
                                       StyleFontDisplay *out);

bool Servo_FontFaceRule_GetFontLanguageOverride(const StyleRawServoFontFaceRule *rule,
                                                StyleFontLanguageOverride *out);

nsAtom *Servo_FontFaceRule_GetFamilyName(const StyleRawServoFontFaceRule *rule);

const StyleUnicodeRange *Servo_FontFaceRule_GetUnicodeRanges(const StyleRawServoFontFaceRule *rule,
                                                             uintptr_t *out_len);

void Servo_FontFaceRule_GetSources(const StyleRawServoFontFaceRule *rule,
                                   nsTArray<StyleFontFaceSourceListComponent> *out);

void Servo_FontFaceRule_GetVariationSettings(const StyleRawServoFontFaceRule *rule,
                                             nsTArray<gfxFontVariation> *variations);

void Servo_FontFaceRule_GetFeatureSettings(const StyleRawServoFontFaceRule *rule,
                                           nsTArray<gfxFontFeature> *features);

void Servo_FontFaceRule_GetDescriptorCssText(const StyleRawServoFontFaceRule *rule,
                                             nsCSSFontDesc desc,
                                             nsACString *result);

bool Servo_FontFaceRule_SetDescriptor(const StyleRawServoFontFaceRule *rule,
                                      nsCSSFontDesc desc,
                                      const nsACString *value,
                                      URLExtraData *data,
                                      bool *out_changed);

void Servo_FontFaceRule_ResetDescriptor(const StyleRawServoFontFaceRule *rule,
                                        nsCSSFontDesc desc);

nsAtom *Servo_CounterStyleRule_GetName(const StyleRawServoCounterStyleRule *rule);

bool Servo_CounterStyleRule_SetName(const StyleRawServoCounterStyleRule *rule,
                                    const nsACString *value);

uint32_t Servo_CounterStyleRule_GetGeneration(const StyleRawServoCounterStyleRule *rule);

bool Servo_CounterStyleRule_GetPad(const StyleRawServoCounterStyleRule *rule,
                                   int32_t *width,
                                   nsString *symbol);

bool Servo_CounterStyleRule_GetPrefix(const StyleRawServoCounterStyleRule *rule,
                                      nsString *out);

bool Servo_CounterStyleRule_GetSuffix(const StyleRawServoCounterStyleRule *rule,
                                      nsString *out);

bool Servo_CounterStyleRule_GetNegative(const StyleRawServoCounterStyleRule *rule,
                                        nsString *prefix,
                                        nsString *suffix);

StyleIsOrdinalInRange Servo_CounterStyleRule_IsInRange(const StyleRawServoCounterStyleRule *rule,
                                                       int32_t ordinal);

void Servo_CounterStyleRule_GetSymbols(const StyleRawServoCounterStyleRule *rule,
                                       StyleOwnedSlice<nsString> *symbols);

void Servo_CounterStyleRule_GetAdditiveSymbols(const StyleRawServoCounterStyleRule *rule,
                                               StyleOwnedSlice<StyleAdditiveSymbol> *symbols);

void Servo_CounterStyleRule_GetSpeakAs(const StyleRawServoCounterStyleRule *rule,
                                       StyleCounterSpeakAs *out);

StyleCounterSystem Servo_CounterStyleRule_GetSystem(const StyleRawServoCounterStyleRule *rule);

nsAtom *Servo_CounterStyleRule_GetExtended(const StyleRawServoCounterStyleRule *rule);

int32_t Servo_CounterStyleRule_GetFixedFirstValue(const StyleRawServoCounterStyleRule *rule);

nsAtom *Servo_CounterStyleRule_GetFallback(const StyleRawServoCounterStyleRule *rule);

StyleStrong<StyleComputedValues> Servo_ComputedValues_GetForAnonymousBox(const StyleComputedValues *parent_style_or_null,
                                                                         PseudoStyleType pseudo,
                                                                         const StyleRawServoStyleSet *raw_data);

StyleStrong<StyleComputedValues> Servo_ResolvePseudoStyle(const StyleRawGeckoElement *element,
                                                          PseudoStyleType pseudo_type,
                                                          bool is_probe,
                                                          const StyleComputedValues *inherited_style,
                                                          const StyleRawServoStyleSet *raw_data);

StyleStrong<StyleComputedValues> Servo_ComputedValues_ResolveXULTreePseudoStyle(const StyleRawGeckoElement *element,
                                                                                nsAtom *pseudo_tag,
                                                                                const StyleComputedValues *inherited_style,
                                                                                const nsTArray<RefPtr<nsAtom>> *input_word,
                                                                                const StyleRawServoStyleSet *raw_data);

void Servo_SetExplicitStyle(const StyleRawGeckoElement *element,
                            const StyleComputedValues *style);

StyleStrong<StyleComputedValues> Servo_ComputedValues_Inherit(const StyleRawServoStyleSet *raw_data,
                                                              PseudoStyleType pseudo,
                                                              const StyleComputedValues *parent_style_context,
                                                              InheritTarget target);

bool Servo_ComputedValues_SpecifiesAnimationsOrTransitions(const StyleComputedValues *values);

void Servo_ComputedValues_GetStyleRuleList(const StyleComputedValues *values,
                                           nsTArray<const StyleRawServoStyleRule*> *rules);

bool Servo_ComputedValues_EqualForCachedAnonymousContentStyle(const StyleComputedValues *a,
                                                              const StyleComputedValues *b);

uint16_t Servo_ComputedValues_BlockifiedDisplay(const StyleComputedValues *style,
                                                bool is_root_element);

StyleRawServoStyleSet *Servo_StyleSet_Init(const StyleDocument *doc);

void Servo_StyleSet_RebuildCachedData(const StyleRawServoStyleSet *raw_data);

void Servo_StyleSet_Drop(StyleRawServoStyleSet *data);

void Servo_StyleSet_CompatModeChanged(const StyleRawServoStyleSet *raw_data);

StyleStrong<StyleRawServoDeclarationBlock> Servo_ParseProperty(nsCSSPropertyID property,
                                                               const nsACString *value,
                                                               URLExtraData *data,
                                                               ParsingMode parsing_mode,
                                                               nsCompatibility quirks_mode,
                                                               StyleLoader *loader,
                                                               uint16_t rule_type);

bool Servo_ParseEasing(const nsACString *easing, nsTimingFunction *output);

void Servo_SerializeEasing(const nsTimingFunction *easing, nsACString *output);

void Servo_GetProperties_Overriding_Animation(const StyleRawGeckoElement *element,
                                              const nsTArray<nsCSSPropertyID> *list,
                                              nsCSSPropertyIDSet *set);

void Servo_MatrixTransform_Operate(StyleMatrixTransformOperator matrix_operator,
                                   const StyleMatrix4x4Components *from,
                                   const StyleMatrix4x4Components *to,
                                   double progress,
                                   StyleMatrix4x4Components *output);

StyleStrong<StyleRawServoDeclarationBlock> Servo_ParseStyleAttribute(const nsACString *data,
                                                                     URLExtraData *raw_extra_data,
                                                                     nsCompatibility quirks_mode,
                                                                     StyleLoader *loader,
                                                                     uint16_t rule_type);

StyleStrong<StyleRawServoDeclarationBlock> Servo_DeclarationBlock_CreateEmpty();

StyleStrong<StyleRawServoDeclarationBlock> Servo_DeclarationBlock_Clone(const StyleRawServoDeclarationBlock *declarations);

bool Servo_DeclarationBlock_Equals(const StyleRawServoDeclarationBlock *a,
                                   const StyleRawServoDeclarationBlock *b);

void Servo_DeclarationBlock_GetCssText(const StyleRawServoDeclarationBlock *declarations,
                                       nsACString *result);

void Servo_DeclarationBlock_SerializeOneValue(const StyleRawServoDeclarationBlock *declarations,
                                              nsCSSPropertyID property_id,
                                              nsACString *buffer,
                                              const StyleComputedValues *computed_values,
                                              const StyleRawServoDeclarationBlock *custom_properties,
                                              const StyleRawServoStyleSet *raw_data);

void Servo_SerializeFontValueForCanvas(const StyleRawServoDeclarationBlock *declarations,
                                       nsACString *buffer);

uint32_t Servo_DeclarationBlock_Count(const StyleRawServoDeclarationBlock *declarations);

bool Servo_DeclarationBlock_GetNthProperty(const StyleRawServoDeclarationBlock *declarations,
                                           uint32_t index,
                                           nsACString *result);

void Servo_DeclarationBlock_GetPropertyValue(const StyleRawServoDeclarationBlock *declarations,
                                             const nsACString *property,
                                             nsACString *value);

void Servo_DeclarationBlock_GetPropertyValueById(const StyleRawServoDeclarationBlock *declarations,
                                                 nsCSSPropertyID property,
                                                 nsACString *value);

bool Servo_DeclarationBlock_GetPropertyIsImportant(const StyleRawServoDeclarationBlock *declarations,
                                                   const nsACString *property);

bool Servo_DeclarationBlock_SetProperty(const StyleRawServoDeclarationBlock *declarations,
                                        const nsACString *property,
                                        const nsACString *value,
                                        bool is_important,
                                        URLExtraData *data,
                                        ParsingMode parsing_mode,
                                        nsCompatibility quirks_mode,
                                        StyleLoader *loader,
                                        uint16_t rule_type,
                                        DeclarationBlockMutationClosure before_change_closure);

bool Servo_DeclarationBlock_SetPropertyToAnimationValue(const StyleRawServoDeclarationBlock *declarations,
                                                        const StyleRawServoAnimationValue *animation_value,
                                                        DeclarationBlockMutationClosure before_change_closure);

bool Servo_DeclarationBlock_SetPropertyById(const StyleRawServoDeclarationBlock *declarations,
                                            nsCSSPropertyID property,
                                            const nsACString *value,
                                            bool is_important,
                                            URLExtraData *data,
                                            ParsingMode parsing_mode,
                                            nsCompatibility quirks_mode,
                                            StyleLoader *loader,
                                            uint16_t rule_type,
                                            DeclarationBlockMutationClosure before_change_closure);

bool Servo_DeclarationBlock_RemoveProperty(const StyleRawServoDeclarationBlock *declarations,
                                           const nsACString *property,
                                           DeclarationBlockMutationClosure before_change_closure);

bool Servo_DeclarationBlock_RemovePropertyById(const StyleRawServoDeclarationBlock *declarations,
                                               nsCSSPropertyID property,
                                               DeclarationBlockMutationClosure before_change_closure);

StyleStrong<StyleRawServoMediaList> Servo_MediaList_Create();

StyleStrong<StyleRawServoMediaList> Servo_MediaList_DeepClone(const StyleRawServoMediaList *list);

bool Servo_MediaList_Matches(const StyleRawServoMediaList *list,
                             const StyleRawServoStyleSet *raw_data);

bool Servo_DeclarationBlock_HasCSSWideKeyword(const StyleRawServoDeclarationBlock *declarations,
                                              nsCSSPropertyID property);

void Servo_MediaList_GetText(const StyleRawServoMediaList *list,
                             nsACString *result);

void Servo_MediaList_SetText(const StyleRawServoMediaList *list,
                             const nsACString *text,
                             StyleCallerType caller_type);

uint32_t Servo_MediaList_GetLength(const StyleRawServoMediaList *list);

bool Servo_MediaList_GetMediumAt(const StyleRawServoMediaList *list,
                                 uint32_t index,
                                 nsACString *result);

void Servo_MediaList_AppendMedium(const StyleRawServoMediaList *list,
                                  const nsACString *new_medium);

bool Servo_MediaList_DeleteMedium(const StyleRawServoMediaList *list,
                                  const nsACString *old_medium);

uintptr_t Servo_MediaList_SizeOfIncludingThis(StyleGeckoMallocSizeOf malloc_size_of,
                                              StyleGeckoMallocSizeOf malloc_enclosing_size_of,
                                              const StyleRawServoMediaList *list);

bool Servo_DeclarationBlock_PropertyIsSet(const StyleRawServoDeclarationBlock *declarations,
                                          nsCSSPropertyID property);

void Servo_DeclarationBlock_SetIdentStringValue(const StyleRawServoDeclarationBlock *declarations,
                                                nsCSSPropertyID property,
                                                nsAtom *value);

void Servo_DeclarationBlock_SetKeywordValue(const StyleRawServoDeclarationBlock *declarations,
                                            nsCSSPropertyID property,
                                            int32_t value);

void Servo_DeclarationBlock_SetIntValue(const StyleRawServoDeclarationBlock *declarations,
                                        nsCSSPropertyID property,
                                        int32_t value);

void Servo_DeclarationBlock_SetMathDepthValue(const StyleRawServoDeclarationBlock *declarations,
                                              int32_t value,
                                              bool is_relative);

void Servo_DeclarationBlock_SetCounterResetListItem(const StyleRawServoDeclarationBlock *declarations,
                                                    int32_t counter_value);

void Servo_DeclarationBlock_SetCounterSetListItem(const StyleRawServoDeclarationBlock *declarations,
                                                  int32_t counter_value);

void Servo_DeclarationBlock_SetPixelValue(const StyleRawServoDeclarationBlock *declarations,
                                          nsCSSPropertyID property,
                                          float value);

void Servo_DeclarationBlock_SetLengthValue(const StyleRawServoDeclarationBlock *declarations,
                                           nsCSSPropertyID property,
                                           float value,
                                           nsCSSUnit unit);

void Servo_DeclarationBlock_SetNumberValue(const StyleRawServoDeclarationBlock *declarations,
                                           nsCSSPropertyID property,
                                           float value);

void Servo_DeclarationBlock_SetPercentValue(const StyleRawServoDeclarationBlock *declarations,
                                            nsCSSPropertyID property,
                                            float value);

void Servo_DeclarationBlock_SetAutoValue(const StyleRawServoDeclarationBlock *declarations,
                                         nsCSSPropertyID property);

void Servo_DeclarationBlock_SetCurrentColor(const StyleRawServoDeclarationBlock *declarations,
                                            nsCSSPropertyID property);

void Servo_DeclarationBlock_SetColorValue(const StyleRawServoDeclarationBlock *declarations,
                                          nsCSSPropertyID property,
                                          nscolor value);

void Servo_DeclarationBlock_SetFontFamily(const StyleRawServoDeclarationBlock *declarations,
                                          const nsACString *value);

void Servo_DeclarationBlock_SetBackgroundImage(const StyleRawServoDeclarationBlock *declarations,
                                               const nsACString *value,
                                               URLExtraData *raw_extra_data);

void Servo_DeclarationBlock_SetTextDecorationColorOverride(const StyleRawServoDeclarationBlock *declarations);

void Servo_DeclarationBlock_SetAspectRatio(const StyleRawServoDeclarationBlock *declarations,
                                           float width,
                                           float height);

bool Servo_CSSSupports2(const nsACString *property, const nsACString *value);

bool Servo_CSSSupports(const nsACString *cond,
                       bool ua_origin,
                       bool chrome_sheet,
                       bool quirks);

void Servo_NoteExplicitHints(const StyleRawGeckoElement *element,
                             StyleRestyleHint restyle_hint,
                             nsChangeHint change_hint);

uint32_t Servo_TakeChangeHint(const StyleRawGeckoElement *element,
                              bool *was_restyled);

StyleStrong<StyleComputedValues> Servo_ResolveStyle(const StyleRawGeckoElement *element);

StyleStrong<StyleComputedValues> Servo_ResolveStyleLazily(const StyleRawGeckoElement *element,
                                                          PseudoStyleType pseudo_type,
                                                          StyleRuleInclusion rule_inclusion,
                                                          const ServoElementSnapshotTable *snapshots,
                                                          const StyleRawServoStyleSet *raw_data);

StyleStrong<StyleComputedValues> Servo_ReparentStyle(const StyleComputedValues *style_to_reparent,
                                                     const StyleComputedValues *parent_style,
                                                     const StyleComputedValues *parent_style_ignoring_first_line,
                                                     const StyleComputedValues *layout_parent_style,
                                                     const StyleRawGeckoElement *element,
                                                     const StyleRawServoStyleSet *raw_data);

void Servo_GetComputedKeyframeValues(const nsTArray<Keyframe> *keyframes,
                                     const StyleRawGeckoElement *element,
                                     PseudoStyleType pseudo_type,
                                     const StyleComputedValues *style,
                                     const StyleRawServoStyleSet *raw_data,
                                     nsTArray<ComputedKeyframeValues> *computed_keyframes);

void Servo_GetAnimationValues(const StyleRawServoDeclarationBlock *declarations,
                              const StyleRawGeckoElement *element,
                              const StyleComputedValues *style,
                              const StyleRawServoStyleSet *raw_data,
                              nsTArray<RefPtr<StyleRawServoAnimationValue>> *animation_values);

nsCSSPropertyID Servo_AnimationValue_GetPropertyId(const StyleRawServoAnimationValue *value);

StyleStrong<StyleRawServoAnimationValue> Servo_AnimationValue_Compute(const StyleRawGeckoElement *element,
                                                                      const StyleRawServoDeclarationBlock *declarations,
                                                                      const StyleComputedValues *style,
                                                                      const StyleRawServoStyleSet *raw_data);

void Servo_AssertTreeIsClean(const StyleRawGeckoElement *root);

bool Servo_IsWorkerThread();

bool Servo_StyleSet_GetKeyframesForName(const StyleRawServoStyleSet *raw_data,
                                        const StyleRawGeckoElement *element,
                                        const StyleComputedValues *style,
                                        nsAtom *name,
                                        const nsTimingFunction *inherited_timing_function,
                                        nsTArray<Keyframe> *keyframes);

void Servo_StyleSet_GetFontFaceRules(const StyleRawServoStyleSet *raw_data,
                                     nsTArray<nsFontFaceRuleContainer> *rules);

const StyleRawServoCounterStyleRule *Servo_StyleSet_GetCounterStyleRule(const StyleRawServoStyleSet *raw_data,
                                                                        nsAtom *name);

gfxFontFeatureValueSet *Servo_StyleSet_BuildFontFeatureValueSet(const StyleRawServoStyleSet *raw_data);

StyleStrong<StyleComputedValues> Servo_StyleSet_ResolveForDeclarations(const StyleRawServoStyleSet *raw_data,
                                                                       const StyleComputedValues *parent_style_context,
                                                                       const StyleRawServoDeclarationBlock *declarations);

void Servo_StyleSet_AddSizeOfExcludingThis(StyleGeckoMallocSizeOf malloc_size_of,
                                           StyleGeckoMallocSizeOf malloc_enclosing_size_of,
                                           ServoStyleSetSizes *sizes,
                                           const StyleRawServoStyleSet *raw_data);

void Servo_UACache_AddSizeOf(StyleGeckoMallocSizeOf malloc_size_of,
                             StyleGeckoMallocSizeOf malloc_enclosing_size_of,
                             ServoStyleSetSizes *sizes);

bool Servo_StyleSet_MightHaveAttributeDependency(const StyleRawServoStyleSet *raw_data,
                                                 const StyleRawGeckoElement *element,
                                                 nsAtom *local_name);

bool Servo_StyleSet_HasStateDependency(const StyleRawServoStyleSet *raw_data,
                                       const StyleRawGeckoElement *element,
                                       uint64_t state);

bool Servo_StyleSet_HasDocumentStateDependency(const StyleRawServoStyleSet *raw_data,
                                               uint64_t state);

void Servo_GetPropertyValue(const StyleComputedValues *style,
                            nsCSSPropertyID prop,
                            nsACString *value);

bool Servo_GetCustomPropertyValue(const StyleComputedValues *computed_values,
                                  const nsACString *name,
                                  nsACString *value);

uint32_t Servo_GetCustomPropertiesCount(const StyleComputedValues *computed_values);

nsAtom *Servo_GetCustomPropertyNameAt(const StyleComputedValues *computed_values,
                                      uint32_t index);

bool Servo_CssUrl_IsLocalRef(const StyleCssUrl *url);

void Servo_ProcessInvalidations(const StyleRawServoStyleSet *set,
                                const StyleRawGeckoElement *element,
                                const ServoElementSnapshotTable *snapshots);

bool Servo_HasPendingRestyleAncestor(const StyleRawGeckoElement *element,
                                     bool may_need_to_flush_layout);

StyleOwnedOrNull<StyleRawServoSelectorList> Servo_SelectorList_Parse(const nsACString *selector_list);

void Servo_SelectorList_Drop(StyleRawServoSelectorList *list);

bool Servo_IsValidCSSColor(const nsACString *value);

bool Servo_ComputeColor(const StyleRawServoStyleSet *raw_data,
                        nscolor current_color,
                        const nsACString *value,
                        nscolor *result_color,
                        bool *was_current_color,
                        StyleLoader *loader);

bool Servo_IntersectionObserverRootMargin_Parse(const nsACString *value,
                                                StyleIntersectionObserverRootMargin *result);

void Servo_IntersectionObserverRootMargin_ToString(const StyleIntersectionObserverRootMargin *root_margin,
                                                   nsACString *result);

bool Servo_ParseTransformIntoMatrix(const nsACString *value,
                                    bool *contain_3d,
                                    StyleMatrix4x4Components *result);

bool Servo_ParseFontShorthandForMatching(const nsACString *value,
                                         URLExtraData *data,
                                         RefPtr<SharedFontList> *family,
                                         StyleComputedFontStyleDescriptor *style,
                                         float *stretch,
                                         float *weight);

StyleOwned<StyleRawServoSourceSizeList> Servo_SourceSizeList_Parse(const nsACString *value);

int32_t Servo_SourceSizeList_Evaluate(const StyleRawServoStyleSet *raw_data,
                                      const StyleRawServoSourceSizeList *list);

void Servo_SourceSizeList_Drop(StyleRawServoSourceSizeList *list);

void Servo_InvalidateStyleForDocStateChanges(const StyleRawGeckoElement *root,
                                             const StyleRawServoStyleSet *document_style,
                                             const nsTArray<const StyleRawServoAuthorStyles*> *non_document_styles,
                                             uint64_t states_changed);

uint64_t Servo_PseudoClass_GetStates(const nsACString *name);

StyleOwned<StyleStyleUseCounters> Servo_UseCounters_Create();

void Servo_UseCounters_Drop(StyleStyleUseCounters *c);

void Servo_UseCounters_Merge(const StyleUseCounters *doc_counters,
                             const StyleUseCounters *sheet_counters);

bool Servo_IsPropertyIdRecordedInUseCounter(const StyleUseCounters *use_counters,
                                            nsCSSPropertyID id);

bool Servo_IsUnknownPropertyRecordedInUseCounter(const StyleUseCounters *use_counters,
                                                 CountedUnknownProperty p);

bool Servo_IsCssPropertyRecordedInUseCounter(const StyleUseCounters *use_counters,
                                             const nsACString *property,
                                             bool *known_prop);

StyleRawServoSharedMemoryBuilder *Servo_SharedMemoryBuilder_Create(uint8_t *buffer,
                                                                   uintptr_t len);

const StyleServoCssRules *Servo_SharedMemoryBuilder_AddStylesheet(StyleRawServoSharedMemoryBuilder *builder,
                                                                  const StyleRawServoStyleSheetContents *raw_contents,
                                                                  nsACString *error_message);

uintptr_t Servo_SharedMemoryBuilder_GetLength(StyleRawServoSharedMemoryBuilder *builder);

void Servo_SharedMemoryBuilder_Drop(StyleRawServoSharedMemoryBuilder *builder);

/// Returns a unique pointer to a clone of the shape image.
///
/// Probably temporary, as we move more stuff to cbindgen.
StyleBasicShape *Servo_CloneBasicShape(const StyleBasicShape *v);

void *Servo_StyleArcSlice_EmptyPtr();

const StyleLoadData *Servo_LoadData_GetLazy(const StyleLoadDataSource *source);

void Servo_LengthPercentage_ToCss(const StyleLengthPercentage *lp,
                                  nsACString *result);

bool Servo_CursorKind_Parse(const nsACString *cursor, StyleCursorKind *result);

#if defined(CBINDGEN_IS_GECKO)
void Servo_ComputedStyle_AddRef(const StyleComputedValues *obj);
#endif

#if defined(CBINDGEN_IS_GECKO)
void Servo_ComputedStyle_Release(const StyleComputedValues *obj);
#endif

} // extern "C"

} // namespace mozilla

#endif // mozilla_ServoStyleConsts_h

#pragma pop_macro("STRICT")
#pragma GCC diagnostic pop
#include "mozilla/ServoStyleConstsInlines.h"
