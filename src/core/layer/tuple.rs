use super::Layer;

impl<H> Layer<H> for () {
    type Handler = H;

    fn layer(&self, handler: H) -> Self::Handler {
        handler
    }
}

impl<H, L1> Layer<H> for (L1,)
where
    L1: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1,) = self;
        l1.layer(handler)
    }
}

impl<H, L1, L2> Layer<H> for (L1, L2)
where
    L1: Layer<L2::Handler>,
    L2: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2) = self;
        l1.layer(l2.layer(handler))
    }
}

impl<H, L1, L2, L3> Layer<H> for (L1, L2, L3)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3) = self;
        l1.layer((l2, l3).layer(handler))
    }
}

impl<H, L1, L2, L3, L4> Layer<H> for (L1, L2, L3, L4)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4) = self;
        l1.layer((l2, l3, l4).layer(handler))
    }
}

impl<H, L1, L2, L3, L4, L5> Layer<H> for (L1, L2, L3, L4, L5)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5) = self;
        l1.layer((l2, l3, l4, l5).layer(handler))
    }
}

impl<H, L1, L2, L3, L4, L5, L6> Layer<H> for (L1, L2, L3, L4, L5, L6)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6) = self;
        l1.layer((l2, l3, l4, l5, l6).layer(handler))
    }
}

impl<H, L1, L2, L3, L4, L5, L6, L7> Layer<H> for (L1, L2, L3, L4, L5, L6, L7)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<L7::Handler>,
    L7: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6, l7) = self;
        l1.layer((l2, l3, l4, l5, l6, l7).layer(handler))
    }
}

impl<H, L1, L2, L3, L4, L5, L6, L7, L8> Layer<H> for (L1, L2, L3, L4, L5, L6, L7, L8)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<L7::Handler>,
    L7: Layer<L8::Handler>,
    L8: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6, l7, l8) = self;
        l1.layer((l2, l3, l4, l5, l6, l7, l8).layer(handler))
    }
}

impl<H, L1, L2, L3, L4, L5, L6, L7, L8, L9> Layer<H> for (L1, L2, L3, L4, L5, L6, L7, L8, L9)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<L7::Handler>,
    L7: Layer<L8::Handler>,
    L8: Layer<L9::Handler>,
    L9: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6, l7, l8, l9) = self;
        l1.layer((l2, l3, l4, l5, l6, l7, l8, l9).layer(handler))
    }
}

impl<H, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10> Layer<H>
    for (L1, L2, L3, L4, L5, L6, L7, L8, L9, L10)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<L7::Handler>,
    L7: Layer<L8::Handler>,
    L8: Layer<L9::Handler>,
    L9: Layer<L10::Handler>,
    L10: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6, l7, l8, l9, l10) = self;
        l1.layer((l2, l3, l4, l5, l6, l7, l8, l9, l10).layer(handler))
    }
}

impl<H, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11> Layer<H>
    for (L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<L7::Handler>,
    L7: Layer<L8::Handler>,
    L8: Layer<L9::Handler>,
    L9: Layer<L10::Handler>,
    L10: Layer<L11::Handler>,
    L11: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11) = self;
        l1.layer((l2, l3, l4, l5, l6, l7, l8, l9, l10, l11).layer(handler))
    }
}

impl<H, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12> Layer<H>
    for (L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<L7::Handler>,
    L7: Layer<L8::Handler>,
    L8: Layer<L9::Handler>,
    L9: Layer<L10::Handler>,
    L10: Layer<L11::Handler>,
    L11: Layer<L12::Handler>,
    L12: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11, l12) = self;
        l1.layer((l2, l3, l4, l5, l6, l7, l8, l9, l10, l11, l12).layer(handler))
    }
}

impl<H, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13> Layer<H>
    for (L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<L7::Handler>,
    L7: Layer<L8::Handler>,
    L8: Layer<L9::Handler>,
    L9: Layer<L10::Handler>,
    L10: Layer<L11::Handler>,
    L11: Layer<L12::Handler>,
    L12: Layer<L13::Handler>,
    L13: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11, l12, l13) = self;
        l1.layer((l2, l3, l4, l5, l6, l7, l8, l9, l10, l11, l12, l13).layer(handler))
    }
}

impl<H, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14> Layer<H>
    for (L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<L7::Handler>,
    L7: Layer<L8::Handler>,
    L8: Layer<L9::Handler>,
    L9: Layer<L10::Handler>,
    L10: Layer<L11::Handler>,
    L11: Layer<L12::Handler>,
    L12: Layer<L13::Handler>,
    L13: Layer<L14::Handler>,
    L14: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11, l12, l13, l14) = self;
        l1.layer((l2, l3, l4, l5, l6, l7, l8, l9, l10, l11, l12, l13, l14).layer(handler))
    }
}

#[rustfmt::skip]
impl<H, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15> Layer<H>
    for (L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<L7::Handler>,
    L7: Layer<L8::Handler>,
    L8: Layer<L9::Handler>,
    L9: Layer<L10::Handler>,
    L10: Layer<L11::Handler>,
    L11: Layer<L12::Handler>,
    L12: Layer<L13::Handler>,
    L13: Layer<L14::Handler>,
    L14: Layer<L15::Handler>,
    L15: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11, l12, l13, l14, l15) = self;
        l1.layer((l2, l3, l4, l5, l6, l7, l8, l9, l10, l11, l12, l13, l14, l15).layer(handler))
    }
}

#[rustfmt::skip]
impl<H, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15, L16> Layer<H>
    for (L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15, L16)
where
    L1: Layer<L2::Handler>,
    L2: Layer<L3::Handler>,
    L3: Layer<L4::Handler>,
    L4: Layer<L5::Handler>,
    L5: Layer<L6::Handler>,
    L6: Layer<L7::Handler>,
    L7: Layer<L8::Handler>,
    L8: Layer<L9::Handler>,
    L9: Layer<L10::Handler>,
    L10: Layer<L11::Handler>,
    L11: Layer<L12::Handler>,
    L12: Layer<L13::Handler>,
    L13: Layer<L14::Handler>,
    L14: Layer<L15::Handler>,
    L15: Layer<L16::Handler>,
    L16: Layer<H>,
{
    type Handler = L1::Handler;

    fn layer(&self, handler: H) -> Self::Handler {
        let (l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, l11, l12, l13, l14, l15, l16) = self;
        l1.layer((l2, l3, l4, l5, l6, l7, l8, l9, l10, l11, l12, l13, l14, l15, l16).layer(handler))
    }
}
