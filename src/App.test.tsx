import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import userEvent from "@testing-library/user-event";
import App from "./App";

const { openUrl } = vi.hoisted(() => ({ openUrl: vi.fn() }));
vi.mock("@tauri-apps/plugin-opener", () => ({ openUrl }));

describe("App", () => {
  it("renders the product name", () => {
    render(<App />);

    expect(
      screen.getByRole("heading", { name: /steam library organizer/i }),
    ).toBeInTheDocument();
  });

  it("opens the smart collection builder", async () => {
    const user = userEvent.setup();
    render(<App />);
    await user.click(screen.getByRole("button", { name: /collections/i }));
    expect(screen.getByRole("heading", { name: /smart collections/i })).toBeInTheDocument();
    expect(screen.getByLabelText(/collection name/i)).toBeInTheDocument();
  });

  it("shows the library workflow", async () => {
    const user = userEvent.setup();
    render(<App />);
    expect(screen.getByRole("heading", { name: /your library/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /use current steam account/i })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /get api key/i })).toHaveAttribute(
      "href",
      "https://steamcommunity.com/dev/apikey",
    );
    await user.click(screen.getByRole("link", { name: /get api key/i }));
    expect(openUrl).toHaveBeenCalledWith("https://steamcommunity.com/dev/apikey");
    expect(screen.getByRole("button", { name: /import steam library/i })).toBeInTheDocument();
  });
});
