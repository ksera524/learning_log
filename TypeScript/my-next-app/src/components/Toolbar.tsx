type ToolbarProps = {
  onBoldClick: () => void;
  onUnderlineClick: () => void;
  onItalicClick: () => void;
};

const Toolbar = ({
  onBoldClick,
  onUnderlineClick,
  onItalicClick,
}: ToolbarProps) => {
  return (
    <div style={{ marginBottom: "10px" }}>
      <button type="button" onClick={onBoldClick}>
        Bold
      </button>
      <button type="button" onClick={onUnderlineClick}>
        Underline
      </button>
      <button type="button" onClick={onItalicClick}>
        Italic
      </button>
    </div>
  );
};

export { Toolbar };
