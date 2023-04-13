import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { SimpleButton } from "./SimpleButton";

test("ボタンをクリックするとON/OFFが切り替わる", async () => {
  render(<SimpleButton />);
  const simpleButton = screen.getByRole("button");
  expect(simpleButton).toHaveTextContent("OFF");
  userEvent.click(simpleButton);
  await waitFor(() => {
    expect(simpleButton).toHaveTextContent("ON");
  });
  userEvent.click(simpleButton);
  await waitFor(() => {
    expect(simpleButton).toHaveTextContent("OFF");
  });
});
