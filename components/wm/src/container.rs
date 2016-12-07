type CWMContainer = CWMC_T;

enum ContainerType {
  Root,
  Output,
  Workspace,
  Container,
  View,
  Types,
}

enum ContainerLayout {
  Horiz,
  Vert,
  Stacked,
  Tabbed,
  Floating,
  Layouts,
}

enum BorderType {
  Pixel,
  Normal,
}

struct Container {
  
