import { render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import userEvent from "@testing-library/user-event";
import App from "./App";

const { openUrl } = vi.hoisted(() => ({ openUrl: vi.fn() }));
vi.mock("@tauri-apps/plugin-opener", () => ({ openUrl }));

describe("App", () => {
  beforeEach(() => {
    localStorage.clear();
    openUrl.mockClear();
  });
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

  it("offers an IP collection suggestion and adds it on request", async () => {
    localStorage.setItem("games", JSON.stringify([{
      appId: 552500,
      name: "Warhammer: Vermintide 2",
      playtimeMinutes: 0,
      installed: false,
      tags: [],
    }]));
    const user = userEvent.setup();
    render(<App />);
    await user.click(screen.getByRole("button", { name: /collections/i }));

    expect(screen.getByRole("heading", { name: /suggested collections/i })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: /add warhammer collection/i }));
    expect(screen.getByText("Warhammer", { selector: "h3" })).toBeInTheDocument();
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
