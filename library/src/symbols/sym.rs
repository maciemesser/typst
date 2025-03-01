use typst::eval::{symbols, Module, Scope, Symbol};

/// A module with all general symbols.
pub fn sym() -> Module {
    let mut scope = Scope::new();
    for (name, symbol) in SYM {
        scope.define(*name, symbol.clone());
    }
    Module::new("sym").with_scope(scope)
}

/// The list of general symbols.
pub(crate) const SYM: &[(&str, Symbol)] = symbols! {
    // Control.
    wj: '\u{2060}',
    zwj: '\u{200D}',
    zwnj: '\u{200C}',
    zws: '\u{200B}',

    // Spaces.
    space: [
        ' ',
        nobreak: '\u{A0}',
        en: '\u{2002}',
        quad: '\u{2003}',
        third: '\u{2004}',
        quarter: '\u{2005}',
        sixth: '\u{2006}',
        med: '\u{205F}',
        fig: '\u{2007}',
        punct: '\u{2008}',
        thin: '\u{2009}',
        hair: '\u{200A}',
    ],

    // Delimiters.
    paren: [l: '(', r: ')', t: '⏜', b: '⏝'],
    brace: [l: '{', r: '}', t: '⏞', b: '⏟'],
    bracket: [l: '[', l.double: '⟦', r: ']', r.double: '⟧', t: '⎴', b: '⎵'],
    turtle: [l: '〔', r: '〕', t: '⏠', b: '⏡'],
    bar: [v: '|', v.double: '‖', v.triple: '⦀', v.broken: '¦', v.circle: '⦶', h: '―'],
    fence: [l: '⧘', l.double: '⧚', r: '⧙', r.double: '⧛', dotted: '⦙'],
    angle: [
        '∠',
        l: '⟨',
        r: '⟩',
        l.double: '《',
        r.double: '》',
        acute: '⦟',
        arc: '∡',
        arc.rev: '⦛',
        rev: '⦣',
        right: '∟',
        right.rev: '⯾',
        right.arc: '⊾',
        right.dot: '⦝',
        right.sq: '⦜',
        spatial: '⟀',
        spheric: '∢',
        spheric.rev: '⦠',
        spheric.top: '⦡',
    ],

    // Punctuation.
    amp: ['&', inv: '⅋'],
    ast: [
        op: '∗',
        basic: '*',
        low: '⁎',
        double: '⁑',
        triple: '⁂',
        small: '﹡',
        circle: '⊛',
        sq: '⧆',
    ],
    at: '@',
    backslash: ['\\', circle: '⦸', not: '⧷'],
    co: '℅',
    colon: [':', eq: '≔', double.eq: '⩴'],
    comma: ',',
    dagger: ['†', double: '‡'],
    dash: [
        en: '–',
        em: '—',
        fig: '‒',
        wave: '〜',
        colon: '∹',
        circle: '⊝',
        wave.double: '〰',
    ],
    dot: [
        op: '⋅',
        basic: '.',
        c: '·',
        circle: '⊙',
        circle.big: '⨀',
        square: '⊡',
        double: '¨',
        triple: '\u{20db}',
        quad: '\u{20dc}',
    ],
    excl: ['!', double: '‼', inv: '¡', quest: '⁉'],
    quest: ['?', double: '⁇', excl: '⁈', inv: '¿'],
    interrobang: '‽',
    hash: '#',
    hyph: ['‐', minus: '\u{2D}', nobreak: '\u{2011}', point: '‧', soft: '\u{ad}'],
    percent: '%',
    copyright: ['©', sound: '℗'],
    permille: '‰',
    pilcrow: ['¶', rev: '⁋'],
    section: '§',
    semi: [';', rev: '⁏'],
    slash: ['/', double: '⫽', triple: '⫻'],
    dots: [h.c: '⋯', h: '…', v: '⋮', down: '⋱', up: '⋰'],
    tilde: [
        op: '∼',
        basic: '~',
        eq: '≃',
        eq.not: '≄',
        eq.rev: '⋍',
        eqq: '≅',
        eqq.not: '≇',
        neqq: '≆',
        not: '≁',
        rev: '∽',
        rev.eqq: '≌',
        triple: '≋',
    ],

    // Accents, quotes, and primes.
    acute: ['´', double: '˝'],
    breve: '˘',
    caret: '‸',
    caron: 'ˇ',
    hat: '^',
    diaer: '¨',
    grave: '`',
    macron: '¯',
    quote: [
        double: '"',
        single: '\'',
        l.double: '“',
        l.single: '‘',
        r.double: '”',
        r.single: '’',
        angle.l.double: '«',
        angle.l.single: '‹',
        angle.r.double: '»',
        angle.r.single: '›',
        high.double: '‟',
        high.single: '‛',
        low.double: '„',
        low.single: '‚',
    ],
    prime: [
        '′',
        rev: '‵',
        double: '″',
        double.rev: '‶',
        triple: '‴',
        triple.rev: '‷',
        quad: '⁗',
    ],

    // https://en.wikipedia.org/wiki/List_of_mathematical_symbols_by_subject
    // Arithmetic.
    plus: [
        '+',
        circle: '⊕',
        circle.arrow: '⟴',
        circle.big: '⨁',
        dot: '∔',
        minus: '±',
        small: '﹢',
        square: '⊞',
        triangle: '⨹',
    ],
    minus: [
        '−',
        circle: '⊖',
        dot: '∸',
        plus: '∓',
        square: '⊟',
        tilde: '≂',
        triangle: '⨺',
    ],
    div: ['÷', circle: '⨸'],
    times: [
        '×',
        big: '⨉',
        circle: '⊗',
        circle.big: '⨂',
        div: '⋇',
        three.l: '⋋',
        three.r: '⋌',
        l: '⋉',
        r: '⋊',
        square: '⊠',
        triangle: '⨻',
    ],
    ratio: '∶',

    // Relations.
    eq: [
        '=',
        star: '≛',
        circle: '⊜',
        colon: '≕',
        def: '≝',
        delta: '≜',
        equi: '≚',
        est: '≙',
        gt: '⋝',
        lt: '⋜',
        m: '≞',
        not: '≠',
        prec: '⋞',
        quest: '≟',
        small: '﹦',
        succ: '⋟',
    ],
    gt: [
        '>',
        circle: '⧁',
        dot: '⋗',
        double: '≫',
        eq: '≥',
        eq.lt: '⋛',
        eq.not: '≱',
        eqq: '≧',
        lt: '≷',
        lt.not: '≹',
        neqq: '≩',
        not: '≯',
        ntilde: '⋧',
        small: '﹥',
        tilde: '≳',
        tilde.not: '≵',
        tri: '⊳',
        tri.eq: '⊵',
        tri.eq.not: '⋭',
        tri.not: '⋫',
        triple: '⋙',
        triple.nested: '⫸',
    ],
    lt: [
        '<',
        circle: '⧀',
        dot: '⋖',
        double: '≪',
        eq: '≤',
        eq.gt: '⋚',
        eq.not: '≰',
        eqq: '≦',
        gt: '≶',
        gt.not: '≸',
        neqq: '≨',
        not: '≮',
        ntilde: '⋦',
        small: '﹤',
        tilde: '≲',
        tilde.not: '≴',
        tri: '⊲',
        tri.eq: '⊴',
        tri.eq.not: '⋬',
        tri.not: '⋪',
        triple: '⋘',
        triple.nested: '⫷',
    ],
    approx: ['≈', eq: '≊', not: '≉'],
    prec: [
        '≺',
        approx: '⪷',
        double: '⪻',
        eq: '≼',
        eq.not: '⋠',
        eqq: '⪳',
        napprox: '⪹',
        neqq: '⪵',
        not: '⊀',
        ntilde: '⋨',
        tilde: '≾',
    ],
    succ: [
        '≻',
        approx: '⪸',
        double: '⪼',
        eq: '≽',
        eq.not: '⋡',
        eqq: '⪴',
        napprox: '⪺',
        neqq: '⪶',
        not: '⊁',
        ntilde: '⋩',
        tilde: '≿',
    ],
    ident: ['≡', not: '≢', strict: '≣'],
    prop: '∝',

    // Set theory.
    nothing: ['∅', rev: '⦰'],
    without: '∖',
    complement: '∁',
    in: [
        '∈',
        not: '∉',
        rev: '∋',
        rev.not: '∌',
        rev.small: '∍',
        small: '∊',
    ],
    subset: [
        '⊂',
        dot: '⪽',
        double: '⋐',
        eq: '⊆',
        eq.not: '⊈',
        eq.sq: '⊑',
        eq.sq.not: '⋢',
        neq: '⊊',
        not: '⊄',
        sq: '⊏',
        sq.neq: '⋤',
    ],
    supset: [
        '⊃',
        dot: '⪾',
        double: '⋑',
        eq: '⊇',
        eq.not: '⊉',
        eq.sq: '⊒',
        eq.sq.not: '⋣',
        neq: '⊋',
        not: '⊅',
        sq: '⊐',
        sq.neq: '⋥',
    ],
    union: [
        '∪',
        arrow: '⊌',
        big: '⋃',
        dot: '⊍',
        dot.big: '⨃',
        double: '⋓',
        minus: '⩁',
        or: '⩅',
        plus: '⊎',
        plus.big: '⨄',
        sq: '⊔',
        sq.big: '⨆',
        sq.double: '⩏',
    ],
    sect: [
        '∩',
        and: '⩄',
        big: '⋂',
        dot: '⩀',
        double: '⋒',
        sq: '⊓',
        sq.big: '⨅',
        sq.double: '⩎',
    ],

    // Calculus.
    infinity: '∞',
    oo: '∞',
    diff: '∂',
    nabla: '∇',
    sum: ['∑', integral: '⨋'],
    product: ['∏', co: '∐'],
    integral: [
        '∫',
        arrow.hook: '⨗',
        ccw: '⨑',
        cont: '∮',
        cont.ccw: '∳',
        cont.cw: '∲',
        cw: '∱',
        double: '∬',
        quad: '⨌',
        sect: '⨙',
        sq: '⨖',
        surf: '∯',
        times: '⨘',
        triple: '∭',
        union: '⨚',
        vol: '∰',
    ],
    laplace: '∆',

    // Logic.
    forall: '∀',
    exists: ['∃', not: '∄'],
    top: '⊤',
    bot: '⊥',
    not: '¬',
    and: ['∧', big: '⋀', curly: '⋏', dot: '⟑', double: '⩓'],
    or: ['∨', big: '⋁', curly: '⋎', dot: '⟇', double: '⩔'],
    models: '⊧',
    therefore: '∴',
    because: '∵',
    qed: '∎',

    // Function and category theory.
    compose: '∘',
    convolve: '∗',
    multimap: '⊸',

    // Number theory.
    divides: ['∣', not: '∤'],
    perp: ['⟂', circle: '⦹'],

    // Algebra.
    wreath: '≀',

    // Geometry.
    parallel: ['∥', circle: '⦷', not: '∦'],

    // Miscellaneous Technical.
    diameter: '⌀',
    join: ['⨝', r: '⟖', l: '⟕', l.r: '⟗'],
    degree: ['°', c: '℃', f: '℉'],
    smash: '⨳',

    // Currency.
    bitcoin: '₿',
    dollar: '$',
    euro: '€',
    franc: '₣',
    lira: '₺',
    peso: '₱',
    pound: '£',
    ruble: '₽',
    rupee: '₹',
    won: '₩',
    yen: '¥',

    // Miscellaneous.
    ballot: ['☐', x: '☒'],
    checkmark: ['✓', light: '🗸'],
    floral: ['❦', l: '☙', r: '❧'],
    notes: [up: '🎜', down: '🎝'],
    refmark: '※',
    servicemark: '℠',
    maltese: '✠',
    suit: [club: '♣', diamond: '♦', heart: '♥', spade: '♠'],

    // Shapes.
    circle: [
        stroked: '○',
        stroked.tiny: '∘',
        stroked.small: '⚬',
        stroked.big: '◯',
        filled: '●',
        filled.tiny: '⦁',
        filled.small: '∙',
        filled.big: '⬤',
        dotted: '◌',
        nested: '⊚',
    ],
    ellipse: [
        stroked.h: '⬭',
        stroked.v: '⬯',
        filled.h: '⬬',
        filled.v: '⬮',
    ],
    triangle: [
        stroked.r: '▷',
        stroked.l: '◁',
        stroked.t: '△',
        stroked.b: '▽',
        stroked.bl: '◺',
        stroked.br: '◿',
        stroked.tl: '◸',
        stroked.tr: '◹',
        stroked.small.r: '▹',
        stroked.small.b: '▿',
        stroked.small.l: '◃',
        stroked.small.t: '▵',
        stroked.rounded: '🛆',
        stroked.nested: '⟁',
        stroked.dot: '◬',
        filled.r: '▶',
        filled.l: '◀',
        filled.t: '▲',
        filled.b: '▼',
        filled.bl: '◣',
        filled.br: '◢',
        filled.tl: '◤',
        filled.tr: '◥',
        filled.small.r: '▸',
        filled.small.b: '▾',
        filled.small.l: '◂',
        filled.small.t: '▴',
    ],
    square: [
        stroked: '□',
        stroked.tiny: '▫',
        stroked.small: '◽',
        stroked.medium: '◻',
        stroked.big: '⬜',
        stroked.dotted: '⬚',
        stroked.rounded: '▢',
        filled: '■',
        filled.tiny: '▪',
        filled.small: '◾',
        filled.medium: '◼',
        filled.big: '⬛',
    ],
    rect: [
        stroked.h: '▭',
        stroked.v: '▯',
        filled.h: '▬',
        filled.v: '▮',
    ],
    penta: [stroked: '⬠', filled: '⬟'],
    hexa: [stroked: '⬡', filled: '⬢'],
    diamond: [
        stroked: '◇',
        stroked.small: '⋄',
        stroked.medium: '⬦',
        stroked.dot: '⟐',
        filled: '◆',
        filled.medium: '⬥',
        filled.small: '⬩',
    ],
    lozenge: [
        stroked: '◊',
        stroked.small: '⬫',
        stroked.medium: '⬨',
        filled: '⧫',
        filled.small: '⬪',
        filled.medium: '⬧',
    ],
    star: [op: '⋆', stroked: '★', filled: '★'],

    // Arrows, harpoons, and tacks.
    arrow: [
        r: '→',
        r.long.bar: '⟼',
        r.bar: '↦',
        r.curve: '⤷',
        r.dashed: '⇢',
        r.dotted: '⤑',
        r.double: '⇒',
        r.double.bar: '⤇',
        r.double.long: '⟹',
        r.double.long.bar: '⟾',
        r.double.not: '⇏',
        r.filled: '➡',
        r.hook: '↪',
        r.long: '⟶',
        r.long.squiggly: '⟿',
        r.loop: '↬',
        r.not: '↛',
        r.quad: '⭆',
        r.squiggly: '⇝',
        r.stop: '⇥',
        r.stroked: '⇨',
        r.tail: '↣',
        r.triple: '⇛',
        r.twohead.bar: '⤅',
        r.twohead: '↠',
        r.wave: '↝',
        l: '←',
        l.bar: '↤',
        l.curve: '⤶',
        l.dashed: '⇠',
        l.dotted: '⬸',
        l.double: '⇐',
        l.double.bar: '⤆',
        l.double.long: '⟸',
        l.double.long.bar: '⟽',
        l.double.not: '⇍',
        l.filled: '⬅',
        l.hook: '↩',
        l.long: '⟵',
        l.long.bar: '⟻',
        l.long.squiggly: '⬳',
        l.loop: '↫',
        l.not: '↚',
        l.quad: '⭅',
        l.squiggly: '⇜',
        l.stop: '⇤',
        l.stroked: '⇦',
        l.tail: '↢',
        l.triple: '⇚',
        l.twohead.bar: '⬶',
        l.twohead: '↞',
        l.wave: '↜',
        t: '↑',
        t.bar: '↥',
        t.curve: '⤴',
        t.dashed: '⇡',
        t.double: '⇑',
        t.filled: '⬆',
        t.quad: '⟰',
        t.stop: '⤒',
        t.stroked: '⇧',
        t.triple: '⤊',
        t.twohead: '↟',
        b: '↓',
        b.bar: '↧',
        b.curve: '⤵',
        b.dashed: '⇣',
        b.double: '⇓',
        b.filled: '⬇',
        b.quad: '⟱',
        b.stop: '⤓',
        b.stroked: '⇩',
        b.triple: '⤋',
        b.twohead: '↡',
        l.r: '↔',
        l.r.double: '⇔',
        l.r.double.long: '⟺',
        l.r.double.not: '⇎',
        l.r.filled: '⬌',
        l.r.long: '⟷',
        l.r.not: '↮',
        l.r.stroked: '⬄',
        l.r.wave: '↭',
        t.b: '↕',
        t.b.double: '⇕',
        t.b.filled: '⬍',
        t.b.stroked: '⇳',
        tr: '↗',
        tr.double: '⇗',
        tr.filled: '⬈',
        tr.hook: '⤤',
        tr.stroked: '⬀',
        br: '↘',
        br.double: '⇘',
        br.filled: '⬊',
        br.hook: '⤥',
        br.stroked: '⬂',
        tl: '↖',
        tl.double: '⇖',
        tl.filled: '⬉',
        tl.hook: '⤣',
        tl.stroked: '⬁',
        bl: '↙',
        bl.double: '⇙',
        bl.filled: '⬋',
        bl.hook: '⤦',
        bl.stroked: '⬃',
        tl.br: '⤡',
        tr.bl: '⤢',
        ccw: '↺',
        ccw.half: '↶',
        cw: '↻',
        cw.half: '↷',
        zigzag: '↯',
    ],
    arrows: [
        rr: '⇉',
        ll: '⇇',
        tt: '⇈',
        bb: '⇊',
        lr: '⇆',
        lr.stop: '↹',
        rl: '⇄',
        tb: '⇅',
        bt: '⇵',
        rrr: '⇶',
        lll: '⬱',
    ],
    arrowhead: [
        t: '⌃',
        b: '⌄',
    ],
    harpoon: [
        rt: '⇀',
        rt.bar: '⥛',
        rt.stop: '⥓',
        rb: '⇁',
        rb.bar: '⥟',
        rb.stop: '⥗',
        lt: '↼',
        lt.bar: '⥚',
        lt.stop: '⥒',
        lb: '↽',
        lb.bar: '⥞',
        lb.stop: '⥖',
        tl: '↿',
        tl.bar: '⥠',
        tl.stop: '⥘',
        tr: '↾',
        tr.bar: '⥜',
        tr.stop: '⥔',
        bl: '⇃',
        bl.bar: '⥡',
        bl.stop: '⥙',
        br: '⇂',
        br.bar: '⥝',
        br.stop: '⥕',
        lt.rt: '⥎',
        lb.rb: '⥐',
        lb.rt: '⥋',
        lt.rb: '⥊',
        tl.bl: '⥑',
        tr.br: '⥏',
        tl.br: '⥍',
        tr.bl: '⥌',
    ],
    harpoons: [
        rtrb: '⥤',
        blbr: '⥥',
        bltr: '⥯',
        lbrb: '⥧',
        ltlb: '⥢',
        ltrb: '⇋',
        ltrt: '⥦',
        rblb: '⥩',
        rtlb: '⇌',
        rtlt: '⥨',
        tlbr: '⥮',
        tltr: '⥣',
    ],
    tack: [
        r: '⊢',
        r.long: '⟝',
        r.double: '⊨',
        l: '⊣',
        l.long: '⟞',
        l.short: '⫞',
        l.double: '⫤',
        t: '⊥',
        t.big: '⟘',
        t.double: '⫫',
        t.short: '⫠',
        b: '⊤',
        b.big: '⟙',
        b.double: '⫪',
        b.short: '⫟',
        l.r: '⟛',
    ],

     // Lowercase Greek.
     alpha: 'α',
     beta: ['β', alt: 'ϐ'],
     chi: 'χ',
     delta: 'δ',
     epsilon: ['ε', alt: 'ϵ'],
     eta: 'η',
     gamma: 'γ',
     iota: 'ι',
     kai: 'ϗ',
     kappa: ['κ', alt: 'ϰ'],
     lambda: 'λ',
     mu: 'μ',
     nu: 'ν',
     ohm: ['Ω', inv: '℧'],
     omega: 'ω',
     omicron: 'ο',
     phi: ['φ', alt: 'ϕ'],
     pi: ['π', alt: 'ϖ'],
     psi: 'ψ',
     rho: ['ρ', alt: 'ϱ'],
     sigma: 'σ',
     tau: 'τ',
     theta: ['θ', alt: 'ϑ'],
     upsilon: 'υ',
     xi: 'ξ',
     zeta: 'ζ',

     // Uppercase Greek.
     Alpha: 'Α',
     Beta: 'Β',
     Chi: 'Χ',
     Delta: 'Δ',
     Epsilon: 'Ε',
     Eta: 'Η',
     Gamma: 'Γ',
     Iota: 'Ι',
     Kai: 'Ϗ',
     Kappa: 'Κ',
     Lambda: 'Λ',
     Mu: 'Μ',
     Nu: 'Ν',
     Omega: 'Ω',
     Omicron: 'Ο',
     Phi: 'Φ',
     Pi: 'Π',
     Psi: 'Ψ',
     Rho: 'Ρ',
     Sigma: 'Σ',
     Tau: 'Τ',
     Theta: 'Θ',
     Upsilon: 'Υ',
     Xi: 'Ξ',
     Zeta: 'Ζ',

     // Hebrew.
     alef: 'א',
     bet: 'ב',
     gimel: 'ג',
     shin: 'ש',

     // Double-struck.
     AA: '𝔸',
     BB: '𝔹',
     CC: 'ℂ',
     DD: '𝔻',
     EE: '𝔼',
     FF: '𝔽',
     GG: '𝔾',
     HH: 'ℍ',
     II: '𝕀',
     JJ: '𝕁',
     KK: '𝕂',
     LL: '𝕃',
     MM: '𝕄',
     NN: 'ℕ',
     OO: '𝕆',
     PP: 'ℙ',
     QQ: 'ℚ',
     RR: 'ℝ',
     SS: '𝕊',
     TT: '𝕋',
     UU: '𝕌',
     VV: '𝕍',
     WW: '𝕎',
     XX: '𝕏',
     YY: '𝕐',
     ZZ: 'ℤ',

     // Miscellaneous letter-likes.
     ell: 'ℓ',
     planck: ['ℎ', reduce: 'ℏ'],
     angstrom: 'Å',
     kelvin: 'K',
     Re: 'ℜ',
     Im: 'ℑ',
     dotless: [i: '𝚤', j: '𝚥'],
};
