/** Persistencia de workspace y paneles por workspace (SPEC §6). */

export type WorkspaceId =
  | "compare"
  | "inspect"
  | "audio"
  | "report"
  | "export";

export interface DisposicionPaneles {
  izquierdoVisible: boolean;
  derechoVisible: boolean;
  anchoIzquierdoPx: number;
  anchoDerechoPx: number;
}

export interface EstadoLayout {
  workspaceActivo: WorkspaceId;
  porWorkspace: Record<WorkspaceId, DisposicionPaneles>;
}

const CLAVE_STORAGE = "diffplayerqc-v2-layout";

const POR_DEFECTO: DisposicionPaneles = {
  izquierdoVisible: true,
  derechoVisible: true,
  anchoIzquierdoPx: 260,
  anchoDerechoPx: 300,
};

function disposicionInicial(): Record<WorkspaceId, DisposicionPaneles> {
  return {
    compare: { ...POR_DEFECTO },
    inspect: { ...POR_DEFECTO, anchoDerechoPx: 280 },
    audio: {
      izquierdoVisible: true,
      derechoVisible: false,
      anchoIzquierdoPx: 240,
      anchoDerechoPx: 0,
    },
    report: {
      izquierdoVisible: false,
      derechoVisible: false,
      anchoIzquierdoPx: 0,
      anchoDerechoPx: 0,
    },
    export: {
      izquierdoVisible: false,
      derechoVisible: false,
      anchoIzquierdoPx: 0,
      anchoDerechoPx: 0,
    },
  };
}

function cargar(): EstadoLayout {
  if (typeof localStorage === "undefined") {
    return {
      workspaceActivo: "compare",
      porWorkspace: disposicionInicial(),
    };
  }
  try {
    const raw = localStorage.getItem(CLAVE_STORAGE);
    if (!raw) {
      return {
        workspaceActivo: "compare",
        porWorkspace: disposicionInicial(),
      };
    }
    const parsed = JSON.parse(raw) as EstadoLayout;
    return {
      workspaceActivo: parsed.workspaceActivo ?? "compare",
      porWorkspace: { ...disposicionInicial(), ...parsed.porWorkspace },
    };
  } catch {
    return {
      workspaceActivo: "compare",
      porWorkspace: disposicionInicial(),
    };
  }
}

function guardar(estado: EstadoLayout) {
  if (typeof localStorage === "undefined") return;
  localStorage.setItem(CLAVE_STORAGE, JSON.stringify(estado));
}

class LayoutStore {
  workspaceActivo = $state<WorkspaceId>("compare");
  porWorkspace = $state<Record<WorkspaceId, DisposicionPaneles>>(
    disposicionInicial(),
  );

  constructor() {
    const inicial = cargar();
    this.workspaceActivo = inicial.workspaceActivo;
    this.porWorkspace = inicial.porWorkspace;
  }

  disposicionActual(): DisposicionPaneles {
    return this.porWorkspace[this.workspaceActivo];
  }

  cambiarWorkspace(id: WorkspaceId) {
    this.workspaceActivo = id;
    this.persistir();
  }

  alternarPanel(lado: "izquierdo" | "derecho") {
    const d = { ...this.disposicionActual() };
    if (lado === "izquierdo") {
      d.izquierdoVisible = !d.izquierdoVisible;
    } else {
      d.derechoVisible = !d.derechoVisible;
    }
    this.porWorkspace[this.workspaceActivo] = d;
    this.persistir();
  }

  persistir() {
    guardar({
      workspaceActivo: this.workspaceActivo,
      porWorkspace: this.porWorkspace,
    });
  }
}

export const layoutStore = new LayoutStore();
