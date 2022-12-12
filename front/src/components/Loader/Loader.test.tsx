import React from "react";
import { render, screen } from "@testing-library/react";
import Loader, { Stage } from "./Loader";

test("Loader component create", () => {
  render(<Loader />);
  const rootElement = screen.getByTestId("loader-element");
  expect(rootElement).toBeInTheDocument();
  expect(rootElement).toHaveClass("loader");
  const circleElement = screen.getByTestId("loader-circle-element");
  expect(circleElement).toBeInTheDocument();
  expect(circleElement).toHaveClass("circle");
});

test("Loader component create with inactive stage", () => {
  render(<Loader stage={ Stage.inactive } />);
  const rootElement = screen.getByTestId("loader-element");
  expect(rootElement).toBeInTheDocument();
  expect(rootElement).toHaveClass("loader");
  const circleElement = screen.getByTestId("loader-circle-element");
  expect(circleElement).toBeInTheDocument();
  expect(circleElement).toHaveClass("circle");
  expect(circleElement).not.toHaveClass("active");
  expect(circleElement).not.toHaveClass("complete");
});

test("Loader component create with active stage", () => {
  render(<Loader stage={ Stage.active } />);
  const rootElement = screen.getByTestId("loader-element");
  expect(rootElement).toBeInTheDocument();
  expect(rootElement).toHaveClass("loader");
  const circleElement = screen.getByTestId("loader-circle-element");
  expect(circleElement).toBeInTheDocument();
  expect(circleElement).toHaveClass("circle");
  expect(circleElement).toHaveClass("active");
  expect(circleElement).not.toHaveClass("complete");
});

test("Loader component create with complete stage", () => {
  render(<Loader stage={ Stage.complete } />);
  const rootElement = screen.getByTestId("loader-element");
  expect(rootElement).toBeInTheDocument();
  expect(rootElement).toHaveClass("loader");
  const circleElement = screen.getByTestId("loader-circle-element");
  expect(circleElement).toBeInTheDocument();
  expect(circleElement).toHaveClass("circle");
  expect(circleElement).not.toHaveClass("active");
  expect(circleElement).toHaveClass("complete");
});

test("Loader component create with color", () => {
  const color = "shadow-green-500";
  render(<Loader color={ color } />);
  const rootElement = screen.getByTestId("loader-element");
  expect(rootElement).toBeInTheDocument();
  expect(rootElement).toHaveClass("loader");
  const circleElement = screen.getByTestId("loader-circle-element");
  expect(circleElement).toBeInTheDocument();
  expect(circleElement).toHaveClass("circle");
  expect(circleElement).not.toHaveClass("active");
  expect(circleElement).toHaveClass(color);
});
