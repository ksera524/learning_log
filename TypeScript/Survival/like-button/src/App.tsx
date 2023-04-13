import "./App.css";
import { useState } from "react";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <LikeButton />
      </header>
    </div>
  );
}

function LikeButton() {
  const [count, setCount] = useState(0);

  const onClick = () => {
    setCount(count + 1);
  };

  return <span onClick={onClick}>♥ {count}</span>;
}

export default App;
