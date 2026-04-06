use crate::belt_promotion_exam::belt_promotion_exam_controller::BeltPromotionExamController;

pub enum Controllers {
    BPEController(BeltPromotionExamController),
    None
}