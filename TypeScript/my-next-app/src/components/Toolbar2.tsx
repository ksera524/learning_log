import React from "react";

interface ToolbarProps {
  toggleStyle: (styleType: "inline" | "block", style: string) => void;
  applyFontSize: (fontSize: string) => void;
  applyFontFamily: (fontFamily: string) => void;
  addLink: () => void;
  textAlignLeft: () => void;
  textAlignCenter: () => void;
  textAlignRight: () => void;
  increaseIndent: () => void;
  decreaseIndent: () => void;
  currentFontSize: string;
  currentFontFamily: string;
}

const Toolbar: React.FC<ToolbarProps> = ({
  toggleStyle,
  applyFontSize,
  applyFontFamily,
  addLink,
  textAlignLeft,
  textAlignCenter,
  textAlignRight,
  increaseIndent,
  decreaseIndent,
  currentFontSize,
  currentFontFamily,
}) => {
  return (
    <div>
      <button onClick={() => toggleStyle("inline", "BOLD")}>Bold</button>
      <button onClick={() => toggleStyle("inline", "ITALIC")}>Italic</button>
      <button onClick={() => toggleStyle("inline", "UNDERLINE")}>
        Underline
      </button>
      <button onClick={() => toggleStyle("block", "unordered-list-item")}>
        Bullet List
      </button>
      <button onClick={() => toggleStyle("block", "ordered-list-item")}>
        Numbered List
      </button>
      <select
        value={currentFontSize}
        onChange={(e) => applyFontSize(e.target.value)}
      >
        <option value="12px">12px</option>
        <option value="16px">16px</option>
        <option value="24px">24px</option>
        <option value="32px">32px</option>
      </select>
      <select
        value={currentFontFamily}
        onChange={(e) => applyFontFamily(e.target.value)}
      >
        <option value="Arial">Arial</option>
        <option value="CourierNew">Courier New</option>
        <option value="Georgia">Georgia</option>
        <option value="TimesNewRoman">Times New Roman</option>
        <option value="MS_Gothic">MS ゴシック</option>
      </select>
      <button onClick={addLink}>Add Link</button>
      <button onClick={textAlignLeft}>Left</button>
      <button onClick={textAlignCenter}>Center</button>
      <button onClick={textAlignRight}>Right</button>
      <button onClick={increaseIndent}>Increase Indent</button>
      <button onClick={decreaseIndent}>Decrease Indent</button>
    </div>
  );
};

export default Toolbar;
