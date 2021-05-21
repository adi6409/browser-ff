//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleRole.idl
//


/// `interface nsIAccessibleRole : nsISupports`
///

/// ```text
/// /**
///  * Defines cross platform (Gecko) roles.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleRole {
    vtable: *const nsIAccessibleRoleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleRole.
unsafe impl XpCom for nsIAccessibleRole {
    const IID: nsIID = nsID(0xad7f32a5, 0x6d5f, 0x4154,
        [0xa5, 0xb8, 0x0f, 0xa7, 0xae, 0xd4, 0x89, 0x36]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleRole {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleRole.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleRoleCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleRole`.
    fn coerce_from(v: &nsIAccessibleRole) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleRoleCoerce for nsIAccessibleRole {
    #[inline]
    fn coerce_from(v: &nsIAccessibleRole) -> &Self {
        v
    }
}

impl nsIAccessibleRole {
    /// Cast this `nsIAccessibleRole` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleRoleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleRole {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIAccessibleRoleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleRole) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleRole
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleRoleVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleRole {
    /// ```text
    /// /**
    ///    * Used when accessible hans't strong defined role.
    ///    */
    /// ```
    ///

    pub const ROLE_NOTHING: i64 = 0;

    /// ```text
    /// /**
    ///    * Represents a title or caption bar for a window. It is used by MSAA only,
    ///    * supported automatically by MS Windows.
    ///    */
    /// ```
    ///

    pub const ROLE_TITLEBAR: i64 = 1;

    /// ```text
    /// /**
    ///    * Represents the menu bar (positioned beneath the title bar of a window)
    ///    * from which menus are selected by the user. The role is used by
    ///    * xul:menubar or role="menubar".
    ///    */
    /// ```
    ///

    pub const ROLE_MENUBAR: i64 = 2;

    /// ```text
    /// /**
    ///    * Represents a vertical or horizontal scroll bar, which is part of the client
    ///    * area or used in a control.
    ///    */
    /// ```
    ///

    pub const ROLE_SCROLLBAR: i64 = 3;

    /// ```text
    /// /**
    ///    * Represents a special mouse pointer, which allows a user to manipulate user
    ///    * interface elements such as windows. For example, a user clicks and drags
    ///    * a sizing grip in the lower-right corner of a window to resize it.
    ///    */
    /// ```
    ///

    pub const ROLE_GRIP: i64 = 4;

    /// ```text
    /// /**
    ///    * Represents a system sound, which is associated with various system events.
    ///    */
    /// ```
    ///

    pub const ROLE_SOUND: i64 = 5;

    /// ```text
    /// /**
    ///    * Represents the system mouse pointer.
    ///    */
    /// ```
    ///

    pub const ROLE_CURSOR: i64 = 6;

    /// ```text
    /// /**
    ///    * Represents the system caret. The role is supported for caret.
    ///    */
    /// ```
    ///

    pub const ROLE_CARET: i64 = 7;

    /// ```text
    /// /**
    ///    * Represents an alert or a condition that a user should be notified about.
    ///    * Assistive Technologies typically respond to the role by reading the entire
    ///    * onscreen contents of containers advertising this role. Should be used for
    ///    * warning dialogs, etc. The role is used by xul:browsermessage,
    ///    * role="alert".
    ///    */
    /// ```
    ///

    pub const ROLE_ALERT: i64 = 8;

    /// ```text
    /// /**
    ///    * Represents the window frame, which contains child objects such as
    ///    * a title bar, client, and other objects contained in a window. The role
    ///    * is supported automatically by MS Windows.
    ///    */
    /// ```
    ///

    pub const ROLE_WINDOW: i64 = 9;

    /// ```text
    /// /**
    ///    * A sub-document (<frame> or <iframe>)
    ///    */
    /// ```
    ///

    pub const ROLE_INTERNAL_FRAME: i64 = 10;

    /// ```text
    /// /**
    ///    * Represents a menu, which presents a list of options from which the user can
    ///    * make a selection to perform an action. It is used for role="menu".
    ///    */
    /// ```
    ///

    pub const ROLE_MENUPOPUP: i64 = 11;

    /// ```text
    /// /**
    ///    * Represents a menu item, which is an entry in a menu that a user can choose
    ///    * to carry out a command, select an option. It is used for xul:menuitem,
    ///    * role="menuitem".
    ///    */
    /// ```
    ///

    pub const ROLE_MENUITEM: i64 = 12;

    /// ```text
    /// /**
    ///    * Represents a ToolTip that provides helpful hints.
    ///    */
    /// ```
    ///

    pub const ROLE_TOOLTIP: i64 = 13;

    /// ```text
    /// /**
    ///    * Represents a main window for an application. It is used for
    ///    * role="application". Also refer to ROLE_APP_ROOT
    ///    */
    /// ```
    ///

    pub const ROLE_APPLICATION: i64 = 14;

    /// ```text
    /// /**
    ///    * Represents a document window. A document window is always contained within
    ///    * an application window. For role="document", see NON_NATIVE_DOCUMENT.
    ///    */
    /// ```
    ///

    pub const ROLE_DOCUMENT: i64 = 15;

    /// ```text
    /// /**
    ///    * Represents a pane within a frame or document window. Users can navigate
    ///    * between panes and within the contents of the current pane, but cannot
    ///    * navigate between items in different panes. Thus, panes represent a level
    ///    * of grouping lower than frame windows or documents, but above individual
    ///    * controls. It is used for the first child of a <frame> or <iframe>.
    ///    */
    /// ```
    ///

    pub const ROLE_PANE: i64 = 16;

    /// ```text
    /// /**
    ///    * Represents a graphical image used to represent data.
    ///    */
    /// ```
    ///

    pub const ROLE_CHART: i64 = 17;

    /// ```text
    /// /**
    ///    * Represents a dialog box or message box. It is used for xul:dialog,
    ///    * role="dialog".
    ///    */
    /// ```
    ///

    pub const ROLE_DIALOG: i64 = 18;

    /// ```text
    /// /**
    ///    * Represents a window border.
    ///    */
    /// ```
    ///

    pub const ROLE_BORDER: i64 = 19;

    /// ```text
    /// /**
    ///    * Logically groups other objects. There is not always a parent-child
    ///    * relationship between the grouping object and the objects it contains. It
    ///    * is used for html:textfield, xul:groupbox, role="group".
    ///    */
    /// ```
    ///

    pub const ROLE_GROUPING: i64 = 20;

    /// ```text
    /// /**
    ///    * Used to visually divide a space into two regions, such as a separator menu
    ///    * item or a bar that divides split panes within a window. It is used for
    ///    * xul:separator, html:hr, role="separator".
    ///    */
    /// ```
    ///

    pub const ROLE_SEPARATOR: i64 = 21;

    /// ```text
    /// /**
    ///    * Represents a toolbar, which is a grouping of controls (push buttons or
        ///    * toggle buttons) that provides easy access to frequently used features. It
    ///    * is used for xul:toolbar, role="toolbar".
    ///    */
    /// ```
    ///

    pub const ROLE_TOOLBAR: i64 = 22;

    /// ```text
    /// /**
    ///    * Represents a status bar, which is an area at the bottom of a window that
    ///    * displays information about the current operation, state of the application,
    ///    * or selected object. The status bar has multiple fields, which display
    ///    * different kinds of information. It is used for xul:statusbar.
    ///    */
    /// ```
    ///

    pub const ROLE_STATUSBAR: i64 = 23;

    /// ```text
    /// /**
    ///    * Represents a table that contains rows and columns of cells, and optionally,
    ///    * row headers and column headers. It is used for html:table,
    ///    * role="grid". Also refer to the following roles: ROLE_COLUMNHEADER,
    ///    * ROLE_ROWHEADER, ROLE_COLUMN, ROLE_ROW, ROLE_CELL.
    ///    */
    /// ```
    ///

    pub const ROLE_TABLE: i64 = 24;

    /// ```text
    /// /**
    ///    * Represents a column header, providing a visual label for a column in
    ///    * a table. It is used for XUL tree column headers, html:th,
    ///    * role="colheader". Also refer to ROLE_TABLE.
    ///    */
    /// ```
    ///

    pub const ROLE_COLUMNHEADER: i64 = 25;

    /// ```text
    /// /**
    ///    * Represents a row header, which provides a visual label for a table row.
    ///    * It is used for role="rowheader". Also, see ROLE_TABLE.
    ///    */
    /// ```
    ///

    pub const ROLE_ROWHEADER: i64 = 26;

    /// ```text
    /// /**
    ///    * Represents a column of cells within a table. Also, see ROLE_TABLE.
    ///    */
    /// ```
    ///

    pub const ROLE_COLUMN: i64 = 27;

    /// ```text
    /// /**
    ///    * Represents a row of cells within a table. Also, see ROLE_TABLE.
    ///    */
    /// ```
    ///

    pub const ROLE_ROW: i64 = 28;

    /// ```text
    /// /**
    ///    * Represents a cell within a table. It is used for html:td and xul:tree cell.
    ///    * Also, see ROLE_TABLE.
    ///    */
    /// ```
    ///

    pub const ROLE_CELL: i64 = 29;

    /// ```text
    /// /**
    ///    * Represents a link to something else. This object might look like text or
    ///    * a graphic, but it acts like a button. It is used for
    ///    * xul:label@class="text-link", html:a, html:area.
    ///    */
    /// ```
    ///

    pub const ROLE_LINK: i64 = 30;

    /// ```text
    /// /**
    ///    * Displays a Help topic in the form of a ToolTip or Help balloon.
    ///    */
    /// ```
    ///

    pub const ROLE_HELPBALLOON: i64 = 31;

    /// ```text
    /// /**
    ///    * Represents a cartoon-like graphic object, such as Microsoft Office
    ///    * Assistant, which is displayed to provide help to users of an application.
    ///    */
    /// ```
    ///

    pub const ROLE_CHARACTER: i64 = 32;

    /// ```text
    /// /**
    ///    * Represents a list box, allowing the user to select one or more items. It
    ///    * is used for xul:listbox, html:select@size, role="list". See also
    ///    * ROLE_LIST_ITEM.
    ///    */
    /// ```
    ///

    pub const ROLE_LIST: i64 = 33;

    /// ```text
    /// /**
    ///    * Represents an item in a list. See also ROLE_LIST.
    ///    */
    /// ```
    ///

    pub const ROLE_LISTITEM: i64 = 34;

    /// ```text
    /// /**
    ///    * Represents an outline or tree structure, such as a tree view control,
    ///    * that displays a hierarchical list and allows the user to expand and
    ///    * collapse branches. Is is used for role="tree".
    ///    */
    /// ```
    ///

    pub const ROLE_OUTLINE: i64 = 35;

    /// ```text
    /// /**
    ///    * Represents an item in an outline or tree structure. It is used for
    ///    * role="treeitem".
    ///    */
    /// ```
    ///

    pub const ROLE_OUTLINEITEM: i64 = 36;

    /// ```text
    /// /**
    ///    * Represents a page tab, it is a child of a page tab list. It is used for
    ///    * xul:tab, role="treeitem". Also refer to ROLE_PAGETABLIST.
    ///    */
    /// ```
    ///

    pub const ROLE_PAGETAB: i64 = 37;

    /// ```text
    /// /**
    ///    * Represents a property sheet. It is used for xul:tabpanel,
    ///    * role="tabpanel".
    ///    */
    /// ```
    ///

    pub const ROLE_PROPERTYPAGE: i64 = 38;

    /// ```text
    /// /**
    ///    * Represents an indicator, such as a pointer graphic, that points to the
    ///    * current item.
    ///    */
    /// ```
    ///

    pub const ROLE_INDICATOR: i64 = 39;

    /// ```text
    /// /**
    ///    * Represents a picture. Is is used for xul:image, html:img.
    ///    */
    /// ```
    ///

    pub const ROLE_GRAPHIC: i64 = 40;

    /// ```text
    /// /**
    ///    * Represents read-only text, such as labels for other controls or
    ///    * instructions in a dialog box. Static text cannot be modified or selected.
    ///    * Is is used for xul:label, xul:description, html:label, role="label".
    ///    */
    /// ```
    ///

    pub const ROLE_STATICTEXT: i64 = 41;

    /// ```text
    /// /**
    ///    * Represents selectable text that allows edits or is designated read-only.
    ///    */
    /// ```
    ///

    pub const ROLE_TEXT_LEAF: i64 = 42;

    /// ```text
    /// /**
    ///    * Represents a push button control. It is used for xul:button, html:button,
    ///    * role="button".
    ///    */
    /// ```
    ///

    pub const ROLE_PUSHBUTTON: i64 = 43;

    /// ```text
    /// /**
    ///    * Represents a check box control. It is used for xul:checkbox,
    ///    * html:input@type="checkbox", role="checkbox".
    ///    */
    /// ```
    ///

    pub const ROLE_CHECKBUTTON: i64 = 44;

    /// ```text
    /// /**
    ///    * Represents an option button, also called a radio button. It is one of a
    ///    * group of mutually exclusive options. All objects sharing a single parent
    ///    * that have this attribute are assumed to be part of single mutually
    ///    * exclusive group. It is used for xul:radio, html:input@type="radio",
    ///    * role="radio".
    ///    */
    /// ```
    ///

    pub const ROLE_RADIOBUTTON: i64 = 45;

    /// ```text
    /// /**
    ///    * Represents a combo box; a popup button with an associated list box that
    ///    * provides a set of predefined choices. It is used for html:select with a
    ///    * size of 1 and xul:menulist. See also ROLE_EDITCOMBOBOX.
    ///    */
    /// ```
    ///

    pub const ROLE_COMBOBOX: i64 = 46;

    /// ```text
    /// /**
    ///    * Represents the calendar control.
    ///    */
    /// ```
    ///

    pub const ROLE_DROPLIST: i64 = 47;

    /// ```text
    /// /**
    ///    * Represents a progress bar, dynamically showing the user the percent
    ///    * complete of an operation in progress. It is used for html:progress,
    ///    * role="progressbar".
    ///    */
    /// ```
    ///

    pub const ROLE_PROGRESSBAR: i64 = 48;

    /// ```text
    /// /**
    ///    * Represents a dial or knob whose purpose is to allow a user to set a value.
    ///    */
    /// ```
    ///

    pub const ROLE_DIAL: i64 = 49;

    /// ```text
    /// /**
    ///    * Represents a hot-key field that allows the user to enter a combination or
    ///    * sequence of keystrokes.
    ///    */
    /// ```
    ///

    pub const ROLE_HOTKEYFIELD: i64 = 50;

    /// ```text
    /// /**
    ///    * Represents a slider, which allows the user to adjust a setting in given
    ///    * increments between minimum and maximum values. It is used by xul:scale,
    ///    * role="slider".
    ///    */
    /// ```
    ///

    pub const ROLE_SLIDER: i64 = 51;

    /// ```text
    /// /**
    ///    * Represents a spin box, which is a control that allows the user to increment
    ///    * or decrement the value displayed in a separate "buddy" control associated
    ///    * with the spin box. It is used for input[type=number] spin buttons.
    ///    */
    /// ```
    ///

    pub const ROLE_SPINBUTTON: i64 = 52;

    /// ```text
    /// /**
    ///    * Represents a graphical image used to diagram data. It is used for svg:svg.
    ///    */
    /// ```
    ///

    pub const ROLE_DIAGRAM: i64 = 53;

    /// ```text
    /// /**
    ///    * Represents an animation control, which contains content that changes over
    ///    * time, such as a control that displays a series of bitmap frames.
    ///    */
    /// ```
    ///

    pub const ROLE_ANIMATION: i64 = 54;

    /// ```text
    /// /**
    ///    * Represents a mathematical equation. It is used by MATHML, where there is a
    ///    * rich DOM subtree for an equation. Use ROLE_FLAT_EQUATION for <img role="math" alt="[TeX]"/>
    ///    */
    /// ```
    ///

    pub const ROLE_EQUATION: i64 = 55;

    /// ```text
    /// /**
    ///    * Represents a button that drops down a list of items.
    ///    */
    /// ```
    ///

    pub const ROLE_BUTTONDROPDOWN: i64 = 56;

    /// ```text
    /// /**
    ///    * Represents a button that drops down a menu.
    ///    */
    /// ```
    ///

    pub const ROLE_BUTTONMENU: i64 = 57;

    /// ```text
    /// /**
    ///    * Represents a button that drops down a grid. It is used for xul:colorpicker.
    ///    */
    /// ```
    ///

    pub const ROLE_BUTTONDROPDOWNGRID: i64 = 58;

    /// ```text
    /// /**
    ///    * Represents blank space between other objects.
    ///    */
    /// ```
    ///

    pub const ROLE_WHITESPACE: i64 = 59;

    /// ```text
    /// /**
    ///    * Represents a container of page tab controls. Is it used for xul:tabs,
    ///    * DHTML: role="tabs". Also refer to ROLE_PAGETAB.
    ///    */
    /// ```
    ///

    pub const ROLE_PAGETABLIST: i64 = 60;

    /// ```text
    /// /**
    ///    * Represents a control that displays time.
    ///    */
    /// ```
    ///

    pub const ROLE_CLOCK: i64 = 61;

    /// ```text
    /// /**
    ///    * Represents a button on a toolbar that has a drop-down list icon directly
    ///    * adjacent to the button.
    ///    */
    /// ```
    ///

    pub const ROLE_SPLITBUTTON: i64 = 62;

    /// ```text
    /// /**
    ///    * Represents an edit control designed for an Internet Protocol (IP) address.
    ///    * The edit control is divided into sections for the different parts of the
    ///    * IP address.
    ///    */
    /// ```
    ///

    pub const ROLE_IPADDRESS: i64 = 63;

    /// ```text
    /// /**
    ///    * Represents a label control that has an accelerator.
    ///    */
    /// ```
    ///

    pub const ROLE_ACCEL_LABEL: i64 = 64;

    /// ```text
    /// /**
    ///    * Represents an arrow in one of the four cardinal directions.
    ///    */
    /// ```
    ///

    pub const ROLE_ARROW: i64 = 65;

    /// ```text
    /// /**
    ///    * Represents a control that can be drawn into and is used to trap events.
    ///    * It is used for html:canvas.
    ///    */
    /// ```
    ///

    pub const ROLE_CANVAS: i64 = 66;

    /// ```text
    /// /**
    ///    * Represents a menu item with a check box.
    ///    */
    /// ```
    ///

    pub const ROLE_CHECK_MENU_ITEM: i64 = 67;

    /// ```text
    /// /**
    ///    * Represents a specialized dialog that lets the user choose a color.
    ///    */
    /// ```
    ///

    pub const ROLE_COLOR_CHOOSER: i64 = 68;

    /// ```text
    /// /**
    ///    * Represents control whose purpose is to allow a user to edit a date.
    ///    */
    /// ```
    ///

    pub const ROLE_DATE_EDITOR: i64 = 69;

    /// ```text
    /// /**
    ///    * An iconified internal frame in an ROLE_DESKTOP_PANE. Also refer to
    ///    * ROLE_INTERNAL_FRAME.
    ///    */
    /// ```
    ///

    pub const ROLE_DESKTOP_ICON: i64 = 70;

    /// ```text
    /// /**
    ///    * A desktop pane. A pane that supports internal frames and iconified
    ///    * versions of those internal frames.
    ///    */
    /// ```
    ///

    pub const ROLE_DESKTOP_FRAME: i64 = 71;

    /// ```text
    /// /**
    ///    * A directory pane. A pane that allows the user to navigate through
    ///    * and select the contents of a directory. May be used by a file chooser.
    ///    * Also refer to ROLE_FILE_CHOOSER.
    ///    */
    /// ```
    ///

    pub const ROLE_DIRECTORY_PANE: i64 = 72;

    /// ```text
    /// /**
    ///    * A file chooser. A specialized dialog that displays the files in the
    ///    * directory and lets the user select a file, browse a different directory,
    ///    * or specify a filename. May use the directory pane to show the contents of
    ///    * a directory. Also refer to ROLE_DIRECTORY_PANE.
    ///    */
    /// ```
    ///

    pub const ROLE_FILE_CHOOSER: i64 = 73;

    /// ```text
    /// /**
    ///    * A font chooser. A font chooser is a component that lets the user pick
    ///    * various attributes for fonts.
    ///    */
    /// ```
    ///

    pub const ROLE_FONT_CHOOSER: i64 = 74;

    /// ```text
    /// /**
    ///    * Frame role. A top level window with a title bar, border, menu bar, etc.
    ///    * It is often used as the primary window for an application.
    ///    */
    /// ```
    ///

    pub const ROLE_CHROME_WINDOW: i64 = 75;

    /// ```text
    /// /**
    ///    *  A glass pane. A pane that is guaranteed to be painted on top of all
    ///    * panes beneath it. Also refer to ROLE_ROOT_PANE.
    ///    */
    /// ```
    ///

    pub const ROLE_GLASS_PANE: i64 = 76;

    /// ```text
    /// /**
    ///    * A document container for HTML, whose children represent the document
    ///    * content.
    ///    */
    /// ```
    ///

    pub const ROLE_HTML_CONTAINER: i64 = 77;

    /// ```text
    /// /**
    ///    * A small fixed size picture, typically used to decorate components.
    ///    */
    /// ```
    ///

    pub const ROLE_ICON: i64 = 78;

    /// ```text
    /// /**
    ///    * Presents an icon or short string in an interface.
    ///    */
    /// ```
    ///

    pub const ROLE_LABEL: i64 = 79;

    /// ```text
    /// /**
    ///    * A layered pane. A specialized pane that allows its children to be drawn
    ///    * in layers, providing a form of stacking order. This is usually the pane
    ///    * that holds the menu bar as  well as the pane that contains most of the
    ///    * visual components in a window. Also refer to ROLE_GLASS_PANE and
    ///    * ROLE_ROOT_PANE.
    ///    */
    /// ```
    ///

    pub const ROLE_LAYERED_PANE: i64 = 80;

    /// ```text
    /// /**
    ///    * A specialized pane whose primary use is inside a dialog.
    ///    */
    /// ```
    ///

    pub const ROLE_OPTION_PANE: i64 = 81;

    /// ```text
    /// /**
    ///    * A text object uses for passwords, or other places where the text content
    ///    * is not shown visibly to the user.
    ///    */
    /// ```
    ///

    pub const ROLE_PASSWORD_TEXT: i64 = 82;

    /// ```text
    /// /**
    ///    * A temporary window that is usually used to offer the user a list of
    ///    * choices, and then hides when the user selects one of those choices.
    ///    */
    /// ```
    ///

    pub const ROLE_POPUP_MENU: i64 = 83;

    /// ```text
    /// /**
    ///    * A radio button that is a menu item.
    ///    */
    /// ```
    ///

    pub const ROLE_RADIO_MENU_ITEM: i64 = 84;

    /// ```text
    /// /**
    ///    * A root pane. A specialized pane that has a glass pane and a layered pane
    ///    * as its children. Also refer to ROLE_GLASS_PANE and ROLE_LAYERED_PANE.
    ///    */
    /// ```
    ///

    pub const ROLE_ROOT_PANE: i64 = 85;

    /// ```text
    /// /**
    ///    * A scroll pane. An object that allows a user to incrementally view a large
    ///    * amount of information.  Its children can include scroll bars and a
    ///    * viewport. Also refer to ROLE_VIEW_PORT.
    ///    */
    /// ```
    ///

    pub const ROLE_SCROLL_PANE: i64 = 86;

    /// ```text
    /// /**
    ///    * A split pane. A specialized panel that presents two other panels at the
    ///    * same time. Between the two panels is a divider the user can manipulate to
    ///    * make one panel larger and the other panel smaller.
    ///    */
    /// ```
    ///

    pub const ROLE_SPLIT_PANE: i64 = 87;

    /// ```text
    /// /**
    ///    * The header for a column of a table.
    ///    * XXX: it looks this role is dupe of ROLE_COLUMNHEADER.
    ///    */
    /// ```
    ///

    pub const ROLE_TABLE_COLUMN_HEADER: i64 = 88;

    /// ```text
    /// /**
    ///    * The header for a row of a table.
    ///    * XXX: it looks this role is dupe of ROLE_ROWHEADER
    ///    */
    /// ```
    ///

    pub const ROLE_TABLE_ROW_HEADER: i64 = 89;

    /// ```text
    /// /**
    ///    * A menu item used to tear off and reattach its menu.
    ///    */
    /// ```
    ///

    pub const ROLE_TEAR_OFF_MENU_ITEM: i64 = 90;

    /// ```text
    /// /**
    ///    * Represents an accessible terminal.
    ///    */
    /// ```
    ///

    pub const ROLE_TERMINAL: i64 = 91;

    /// ```text
    /// /**
    ///    * Collection of objects that constitute a logical text entity.
    ///    */
    /// ```
    ///

    pub const ROLE_TEXT_CONTAINER: i64 = 92;

    /// ```text
    /// /**
    ///    * A toggle button. A specialized push button that can be checked or
    ///    * unchecked, but does not provide a separate indicator for the current state.
    ///    */
    /// ```
    ///

    pub const ROLE_TOGGLE_BUTTON: i64 = 93;

    /// ```text
    /// /**
    ///    * Representas a control that is capable of expanding and collapsing rows as
    ///    * well as showing multiple columns of data.
    ///    * XXX: it looks like this role is dupe of ROLE_OUTLINE.
    ///    */
    /// ```
    ///

    pub const ROLE_TREE_TABLE: i64 = 94;

    /// ```text
    /// /**
    ///    * A viewport. An object usually used in a scroll pane. It represents the
    ///    * portion of the entire data that the user can see. As the user manipulates
    ///    * the scroll bars, the contents of the viewport can change. Also refer to
    ///    * ROLE_SCROLL_PANE.
    ///    */
    /// ```
    ///

    pub const ROLE_VIEWPORT: i64 = 95;

    /// ```text
    /// /**
    ///    * Header of a document page. Also refer to ROLE_FOOTER.
    ///    */
    /// ```
    ///

    pub const ROLE_HEADER: i64 = 96;

    /// ```text
    /// /**
    ///    * Footer of a document page. Also refer to ROLE_HEADER.
    ///    */
    /// ```
    ///

    pub const ROLE_FOOTER: i64 = 97;

    /// ```text
    /// /**
    ///    * A paragraph of text.
    ///    */
    /// ```
    ///

    pub const ROLE_PARAGRAPH: i64 = 98;

    /// ```text
    /// /**
    ///    * A ruler such as those used in word processors.
    ///    */
    /// ```
    ///

    pub const ROLE_RULER: i64 = 99;

    /// ```text
    /// /**
    ///    * A text entry having dialog or list containing items for insertion into
    ///    * an entry widget, for instance a list of words for completion of a
    ///    * text entry. It is used for xul:textbox@autocomplete
    ///    */
    /// ```
    ///

    pub const ROLE_AUTOCOMPLETE: i64 = 100;

    /// ```text
    /// /**
    ///    *  An editable text object in a toolbar.
    ///    */
    /// ```
    ///

    pub const ROLE_EDITBAR: i64 = 101;

    /// ```text
    /// /**
    ///    * An control whose textual content may be entered or modified by the user.
    ///    */
    /// ```
    ///

    pub const ROLE_ENTRY: i64 = 102;

    /// ```text
    /// /**
    ///    * A caption describing another object.
    ///    */
    /// ```
    ///

    pub const ROLE_CAPTION: i64 = 103;

    /// ```text
    /// /**
    ///    * An element containing content that assistive technology users may want to
    ///    * browse in a reading mode, rather than a focus/interactive/application mode.
    ///    * This role is used for role="document". For the container which holds the
    ///    * content of a web page, see ROLE_DOCUMENT.
    ///    */
    /// ```
    ///

    pub const ROLE_NON_NATIVE_DOCUMENT: i64 = 104;

    /// ```text
    /// /**
    ///    * Heading.
    ///    */
    /// ```
    ///

    pub const ROLE_HEADING: i64 = 105;

    /// ```text
    /// /**
    ///    * An object representing a page of document content.  It is used in documents
    ///    * which are accessed by the user on a page by page basis.
    ///    */
    /// ```
    ///

    pub const ROLE_PAGE: i64 = 106;

    /// ```text
    /// /**
    ///    * A container of document content.  An example of the use of this role is to
    ///    * represent an html:div.
    ///    */
    /// ```
    ///

    pub const ROLE_SECTION: i64 = 107;

    /// ```text
    /// /**
    ///    * An object which is redundant with another object in the accessible
    ///    * hierarchy. ATs typically ignore objects with this role.
    ///    */
    /// ```
    ///

    pub const ROLE_REDUNDANT_OBJECT: i64 = 108;

    /// ```text
    /// /**
    ///    * A container of form controls. An example of the use of this role is to
    ///    * represent an html:form.
    ///    */
    /// ```
    ///

    pub const ROLE_FORM: i64 = 109;

    /// ```text
    /// /**
    ///    * An object which is used to allow input of characters not found on a
    ///    * keyboard, such as the input of Chinese characters on a Western keyboard.
    ///    */
    /// ```
    ///

    pub const ROLE_IME: i64 = 110;

    /// ```text
    /// /**
    ///    * XXX: document this.
    ///    */
    /// ```
    ///

    pub const ROLE_APP_ROOT: i64 = 111;

    /// ```text
    /// /**
    ///    * Represents a menu item, which is an entry in a menu that a user can choose
    ///    * to display another menu.
    ///    */
    /// ```
    ///

    pub const ROLE_PARENT_MENUITEM: i64 = 112;

    /// ```text
    /// /**
    ///    * A calendar that allows the user to select a date.
    ///    */
    /// ```
    ///

    pub const ROLE_CALENDAR: i64 = 113;

    /// ```text
    /// /**
    ///    * A list of items that is shown by combobox.
    ///    */
    /// ```
    ///

    pub const ROLE_COMBOBOX_LIST: i64 = 114;

    /// ```text
    /// /**
    ///    * A item of list that is shown by combobox;
    ///    */
    /// ```
    ///

    pub const ROLE_COMBOBOX_OPTION: i64 = 115;

    /// ```text
    /// /**
    ///    * An image map -- has child links representing the areas
    ///    */
    /// ```
    ///

    pub const ROLE_IMAGE_MAP: i64 = 116;

    /// ```text
    /// /**
    ///    * An option in a listbox
    ///    */
    /// ```
    ///

    pub const ROLE_OPTION: i64 = 117;

    /// ```text
    /// /**
    ///    * A rich option in a listbox, it can have other widgets as children
    ///    */
    /// ```
    ///

    pub const ROLE_RICH_OPTION: i64 = 118;

    /// ```text
    /// /**
    ///    * A list of options
    ///    */
    /// ```
    ///

    pub const ROLE_LISTBOX: i64 = 119;

    /// ```text
    /// /**
    ///    * Represents a mathematical equation in the accessible name
    ///    */
    /// ```
    ///

    pub const ROLE_FLAT_EQUATION: i64 = 120;

    /// ```text
    /// /**
    ///    * Represents a cell within a grid. It is used for role="gridcell". Unlike
    ///    * ROLE_CELL, it allows the calculation of the accessible name from subtree.
    ///    * Also, see ROLE_TABLE.
    ///    */
    /// ```
    ///

    pub const ROLE_GRID_CELL: i64 = 121;

    /// ```text
    /// /**
    ///    * Represents an embedded object. It is used for html:object or html:embed.
    ///    */
    /// ```
    ///

    pub const ROLE_EMBEDDED_OBJECT: i64 = 122;

    /// ```text
    /// /**
    ///    * A note. Originally intended to be hidden until activated, but now also used
    ///    * for things like html 'aside'.
    ///    */
    /// ```
    ///

    pub const ROLE_NOTE: i64 = 123;

    /// ```text
    /// /**
    ///    * A figure. Used for things like HTML5 figure element.
    ///    */
    /// ```
    ///

    pub const ROLE_FIGURE: i64 = 124;

    /// ```text
    /// /**
    ///    * Represents a rich item with a check box.
    ///    */
    /// ```
    ///

    pub const ROLE_CHECK_RICH_OPTION: i64 = 125;

    /// ```text
    /// /**
    ///    * An HTML definition list <dl>
    ///    */
    /// ```
    ///

    pub const ROLE_DEFINITION_LIST: i64 = 126;

    /// ```text
    /// /**
    ///    * An HTML definition term <dt>
    ///    */
    /// ```
    ///

    pub const ROLE_TERM: i64 = 127;

    /// ```text
    /// /**
    ///    * An HTML definition <dd>
    ///    */
    /// ```
    ///

    pub const ROLE_DEFINITION: i64 = 128;

    /// ```text
    /// /**
    ///    * A keyboard or keypad key.
    ///    */
    /// ```
    ///

    pub const ROLE_KEY: i64 = 129;

    /// ```text
    /// /**
    ///    * A switch control widget.
    ///    */
    /// ```
    ///

    pub const ROLE_SWITCH: i64 = 130;

    /// ```text
    /// /**
    ///    * A block of MathML code (math).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_MATH: i64 = 131;

    /// ```text
    /// /**
    ///    * A MathML identifier (mi in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_IDENTIFIER: i64 = 132;

    /// ```text
    /// /**
    ///    * A MathML number (mn in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_NUMBER: i64 = 133;

    /// ```text
    /// /**
    ///    * A MathML operator (mo in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_OPERATOR: i64 = 134;

    /// ```text
    /// /**
    ///    * A MathML text (mtext in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_TEXT: i64 = 135;

    /// ```text
    /// /**
    ///    * A MathML string literal (ms in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_STRING_LITERAL: i64 = 136;

    /// ```text
    /// /**
    ///    * A MathML glyph (mglyph in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_GLYPH: i64 = 137;

    /// ```text
    /// /**
    ///    * A MathML row (mrow in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_ROW: i64 = 138;

    /// ```text
    /// /**
    ///    * A MathML fraction (mfrac in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_FRACTION: i64 = 139;

    /// ```text
    /// /**
    ///    * A MathML square root (msqrt in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_SQUARE_ROOT: i64 = 140;

    /// ```text
    /// /**
    ///    * A MathML root (mroot in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_ROOT: i64 = 141;

    /// ```text
    /// /**
    ///    * A MathML fenced element (mfenced in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_FENCED: i64 = 142;

    /// ```text
    /// /**
    ///    * A MathML enclosed element (menclose in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_ENCLOSED: i64 = 143;

    /// ```text
    /// /**
    ///    * A MathML styling element (mstyle in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_STYLE: i64 = 144;

    /// ```text
    /// /**
    ///    * A MathML subscript (msub in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_SUB: i64 = 145;

    /// ```text
    /// /**
    ///    * A MathML superscript (msup in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_SUP: i64 = 146;

    /// ```text
    /// /**
    ///    * A MathML subscript and superscript (msubsup in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_SUB_SUP: i64 = 147;

    /// ```text
    /// /**
    ///    * A MathML underscript (munder in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_UNDER: i64 = 148;

    /// ```text
    /// /**
    ///    * A MathML overscript (mover in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_OVER: i64 = 149;

    /// ```text
    /// /**
    ///    * A MathML underscript and overscript (munderover in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_UNDER_OVER: i64 = 150;

    /// ```text
    /// /**
    ///    * A MathML multiple subscript and superscript element (mmultiscripts in
        ///    * MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_MULTISCRIPTS: i64 = 151;

    /// ```text
    /// /**
    ///    * A MathML table (mtable in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_TABLE: i64 = 152;

    /// ```text
    /// /**
    ///    * A MathML labelled table row (mlabeledtr in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_LABELED_ROW: i64 = 153;

    /// ```text
    /// /**
    ///    * A MathML table row (mtr in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_TABLE_ROW: i64 = 154;

    /// ```text
    /// /**
    ///    * A MathML table entry or cell (mtd in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_CELL: i64 = 155;

    /// ```text
    /// /**
    ///    * A MathML interactive element (maction in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_ACTION: i64 = 156;

    /// ```text
    /// /**
    ///    * A MathML error message (merror in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_ERROR: i64 = 157;

    /// ```text
    /// /**
    ///    * A MathML stacked (rows of numbers) element (mstack in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_STACK: i64 = 158;

    /// ```text
    /// /**
    ///    * A MathML long division element (mlongdiv in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_LONG_DIVISION: i64 = 159;

    /// ```text
    /// /**
    ///    * A MathML stack group (msgroup in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_STACK_GROUP: i64 = 160;

    /// ```text
    /// /**
    ///    * A MathML stack row (msrow in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_STACK_ROW: i64 = 161;

    /// ```text
    /// /**
    ///    * MathML carries, borrows, or crossouts for a row (mscarries in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_STACK_CARRIES: i64 = 162;

    /// ```text
    /// /**
    ///    * A MathML carry, borrow, or crossout for a column (mscarry in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_STACK_CARRY: i64 = 163;

    /// ```text
    /// /**
    ///    * A MathML line in a stack (msline in MathML).
    ///    */
    /// ```
    ///

    pub const ROLE_MATHML_STACK_LINE: i64 = 164;

    /// ```text
    /// /**
    ///    * A group containing radio buttons
    ///    */
    /// ```
    ///

    pub const ROLE_RADIO_GROUP: i64 = 165;

    /// ```text
    /// /**
    ///    * A text container exposing brief amount of information. See related
    ///    * TEXT_CONTAINER role.
    ///    */
    /// ```
    ///

    pub const ROLE_TEXT: i64 = 166;

    /// ```text
    /// /**
    ///    * A text container exposing brief amount of information. See related
    ///    * DETAILS role.
    ///    */
    /// ```
    ///

    pub const ROLE_DETAILS: i64 = 167;

    /// ```text
    /// /**
    ///    * A text container exposing brief amount of information. See related
    ///    * SUMMARY role.
    ///    */
    /// ```
    ///

    pub const ROLE_SUMMARY: i64 = 168;

    /// ```text
    /// /**
    ///    * An ARIA landmark. See related NAVIGATION role.
    ///    */
    /// ```
    ///

    pub const ROLE_LANDMARK: i64 = 169;

    /// ```text
    /// /**
    ///    * A specific type of ARIA landmark. The ability to distinguish navigation
    ///    * landmarks from other types of landmarks is needed because macOS has a
    ///    * specific AXSubrole and AXRoleDescription for navigation landmarks.
    ///    */
    /// ```
    ///

    pub const ROLE_NAVIGATION: i64 = 170;

    /// ```text
    /// /**
    ///    * An object that contains the text of a footnote.
    ///    */
    /// ```
    ///

    pub const ROLE_FOOTNOTE: i64 = 171;

    /// ```text
    /// /**
    ///    * A complete or self-contained composition in a document, page, application,
    ///    * or site and that is, in principle, independently distributable or reusable,
    ///    * e.g. in syndication.
    ///    */
    /// ```
    ///

    pub const ROLE_ARTICLE: i64 = 172;

    /// ```text
    /// /**
    ///    * A perceivable section containing content that is relevant to a specific,
    ///    * author-specified purpose and sufficiently important that users will likely
    ///    * want to be able to navigate to the section easily and to have it listed in
    ///    * a summary of the page.
    ///    */
    /// ```
    ///

    pub const ROLE_REGION: i64 = 173;

    /// ```text
    /// /**
    ///    * Represents a control with a text input and a popup with a set of predefined
    ///    * choices. It is used for ARIA's combobox role. See also ROLE_COMBOBOX.
    ///    */
    /// ```
    ///

    pub const ROLE_EDITCOMBOBOX: i64 = 174;

    /// ```text
    /// /**
    ///    * A section of content that is quoted from another source.
    ///    */
    /// ```
    ///

    pub const ROLE_BLOCKQUOTE: i64 = 175;

    /// ```text
    /// /**
    ///    * Content previously deleted or proposed for deletion, e.g. in revision
    ///    * history or a content view providing suggestions from reviewers.
    ///    */
    /// ```
    ///

    pub const ROLE_CONTENT_DELETION: i64 = 176;

    /// ```text
    /// /**
    ///    * Content previously inserted or proposed for insertion, e.g. in revision
    ///    * history or a content view providing suggestions from reviewers.
    ///    */
    /// ```
    ///

    pub const ROLE_CONTENT_INSERTION: i64 = 177;

    /// ```text
    /// /**
    ///    * An html:form element with a label provided by WAI-ARIA.
    ///    * This may also be used if role="form" with a label should be exposed
    ///    * differently in the future.
    ///    */
    /// ```
    ///

    pub const ROLE_FORM_LANDMARK: i64 = 178;

    /// ```text
    /// /**
    ///    * The html:mark element.
    ///    * May also be used if WAI-ARIA gets an equivalent role.
    ///    */
    /// ```
    ///

    pub const ROLE_MARK: i64 = 179;

    /// ```text
    /// /**
    ///    * The WAI-ARIA suggestion role.
    ///    */
    /// ```
    ///

    pub const ROLE_SUGGESTION: i64 = 180;

    /// ```text
    /// /**
    ///    * The WAI-ARIA comment role.
    ///    */
    /// ```
    ///

    pub const ROLE_COMMENT: i64 = 181;

    /// ```text
    /// /**
    ///    * A snippet of program code. ATs might want to treat this differently.
    ///    */
    /// ```
    ///

    pub const ROLE_CODE: i64 = 182;

    /// ```text
    /// /**
    ///    * Represents control whose purpose is to allow a user to edit a time.
    ///    */
    /// ```
    ///

    pub const ROLE_TIME_EDITOR: i64 = 183;

    /// ```text
    /// /**
    ///    * Represents the marker associated with a list item. In unordered lists,
    ///    * this is a bullet, while in ordered lists this is a number.
    ///    */
    /// ```
    ///

    pub const ROLE_LISTITEM_MARKER: i64 = 184;

    /// ```text
    /// /**
    ///    * Essentially, this is a progress bar with a contextually defined
    ///    * scale, ex. the strength of a password entered in an input.
    ///    */
    /// ```
    ///

    pub const ROLE_METER: i64 = 185;


}


